use crate::{notification_type::NotificationType, object::Id};
use crate::{prelude::*, AppNotification};
use cocoa::appkit::{NSApp, NSApplication};
use crossbeam::channel::Sender;
use lazy_static::lazy_static;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Object, Sel},
    sel, sel_impl,
};
use objc::{Encode, Encoding};
use std::sync::atomic::AtomicBool;

lazy_static! {
    static ref APP_WATCHER_DECLARED: AtomicBool = AtomicBool::new(false);
}

struct SenderWrap<T>(*const Sender<T>);

unsafe impl<T> Encode for SenderWrap<T> {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("?") }
    }
}

fn ensure_declared<T>() {
    if APP_WATCHER_DECLARED.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    let mut cls = ClassDecl::new(NSAppWatcher::class_name(), class!(NSObject)).unwrap();

    unsafe {
        cls.add_method(
            sel!(applicationWillLaunch:),
            application_will_launch as extern "C" fn(&Object, Sel, Id),
        );
        cls.add_method(
            sel!(applicationLaunched:),
            application_launched as extern "C" fn(&Object, Sel, Id),
        );
        cls.add_method(
            sel!(applicationTerminated:),
            application_terminated as extern "C" fn(&Object, Sel, Id),
        );
        cls.add_method(
            sel!(applicationHidden:),
            application_hidden as extern "C" fn(&Object, Sel, Id),
        );
        cls.add_method(
            sel!(applicationUnhidden:),
            application_unhidden as extern "C" fn(&Object, Sel, Id),
        );
        cls.add_method(
            sel!(applicationActivated:),
            application_activated as extern "C" fn(&Object, Sel, Id),
        );
        cls.add_method(
            sel!(applicationDeactivated:),
            application_deactivated as extern "C" fn(&Object, Sel, Id),
        );

        // cls.add_ivar::<AppWatcherEvent>("callbackFunctionPtr");
        cls.add_ivar::<SenderWrap<T>>("channelSenderPtr");
    }

    cls.register();

    APP_WATCHER_DECLARED.store(true, std::sync::atomic::Ordering::Relaxed);
}

fn invoke_sender(app_watcher: &Object, notification_type: NotificationType, notification: Id) {
    unsafe {
        let app_watcher: Id = app_watcher as *const _ as Id;
        let mut watcher = NSAppWatcher::from_ptr(app_watcher).unwrap();

        let app_notification =
            AppNotification::parse_notification(notification, notification_type).unwrap();

        let sender = watcher.get_sender();
        let sender: &Sender<AppNotification> = &*(sender.0);
        sender.send(app_notification).unwrap();
    }
}

extern "C" fn application_will_launch(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Launching, notification);
}

extern "C" fn application_launched(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Launched, notification);
}

extern "C" fn application_terminated(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Terminated, notification);
}

extern "C" fn application_hidden(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Hidden, notification);
}

extern "C" fn application_unhidden(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Unhidden, notification);
}

extern "C" fn application_activated(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Activated, notification);
}

extern "C" fn application_deactivated(app_watcher: &Object, _cmd: Sel, notification: Id) {
    invoke_sender(app_watcher, NotificationType::Deactivated, notification);
}

#[derive(CocoaType)]
pub struct NSAppWatcher {
    ptr: Id,
}

#[allow(clippy::new_without_default)]
impl NSAppWatcher {
    unsafe fn set_sender<T>(&mut self, sender: SenderWrap<T>) {
        let ivar = self
            .ptr
            .as_mut()
            .expect("Could not get reference to AppWatcher")
            .get_mut_ivar::<SenderWrap<T>>("channelSenderPtr");

        *ivar = sender;
    }

    unsafe fn get_sender<T>(&mut self) -> &mut SenderWrap<T> {
        return self
            .ptr
            .as_mut()
            .expect("Could not get reference to AppWatcher")
            .get_mut_ivar::<SenderWrap<T>>("channelSenderPtr");
    }

    fn run(&mut self) {
        unsafe {
            let app = NSApp();
            app.run();
        };
    }

    pub fn start_with_sender<T>(sender: Sender<T>)
    where
        T: From<AppNotification>,
    {
        unsafe {
            ensure_declared::<T>();

            // get pointer to sender
            let sender_ptr: *const Sender<T> = &sender;
            let sender_wrap = SenderWrap(sender_ptr);

            let ptr: Id = msg_send![class!(NSAppWatcher), new];
            let mut watcher = NSAppWatcher { ptr };

            watcher.set_sender(sender_wrap);

            watcher.register_callbacks();
            watcher.run();
        }
    }

    pub fn register_callbacks(&mut self) {
        unsafe {
            let shared_workspace = Workspace::shared_workspace();

            let mut notification_center = shared_workspace.notification_center();

            notification_center.add_observer(self, NotificationType::Activated, None);
            notification_center.add_observer(self, NotificationType::Deactivated, None);
            notification_center.add_observer(self, NotificationType::Hidden, None);
            notification_center.add_observer(self, NotificationType::Unhidden, None);
            notification_center.add_observer(self, NotificationType::Launched, None);
            notification_center.add_observer(self, NotificationType::Launching, None);
            notification_center.add_observer(self, NotificationType::Terminated, None);
        }
    }

    pub fn unregister_callbacks(&mut self) {
        unsafe {
            let shared_workspace = Workspace::shared_workspace();

            let mut notification_center = shared_workspace.notification_center();

            notification_center.remove_observer(self, NotificationType::Activated);
            notification_center.remove_observer(self, NotificationType::Deactivated);
            notification_center.remove_observer(self, NotificationType::Hidden);
            notification_center.remove_observer(self, NotificationType::Unhidden);
            notification_center.remove_observer(self, NotificationType::Launched);
            notification_center.remove_observer(self, NotificationType::Launching);
            notification_center.remove_observer(self, NotificationType::Terminated);
        }
    }

    pub fn is_running(&self) -> bool {
        false
    }

    pub fn is_paused(&self) -> bool {
        false
    }
}
