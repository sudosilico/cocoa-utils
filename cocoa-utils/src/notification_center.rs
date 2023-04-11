use cocoa::{base::nil, foundation::NSAutoreleasePool};

use crate::{prelude::*, NSAppWatcher, NotificationType};

#[derive(CocoaType)]
pub struct NotificationCenter {
    ptr: Id,
}

#[allow(clippy::missing_safety_doc)]
impl NotificationCenter {
    #[cocoa_type_property(NSNotificationCenter, defaultCenter)]
    pub unsafe fn default_center() -> Option<NotificationCenter> {}

    pub unsafe fn add_observer(
        &mut self,
        observer: &NSAppWatcher,
        notification_type: NotificationType,
        object: Option<&URL>,
    ) {
        let name = NS_String::from(notification_type.get_name());
        let object = object.map(|url| url.ptr()).unwrap_or(nil);
        let selector = notification_type.get_sel();
        let observer = observer.ptr();
        let _: () =
            msg_send![self.ptr(), addObserver:observer selector:selector name:name object:object];
    }

    pub fn remove_observer(
        &mut self,
        observer: &NSAppWatcher,
        notification_type: NotificationType,
    ) {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);
            let name = NS_String::from(notification_type.get_name());
            let _: () = msg_send![self.ptr(), removeObserver:observer name:name object:nil];
            pool.drain();
        }
    }
}
