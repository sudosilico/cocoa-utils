use crate::prelude::*;
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use objc::{msg_send, sel, sel_impl};

#[derive(Debug, Clone)]
pub struct RunningAppInfo {
    pub pid: usize,
    pub localized_name: Option<String>,
    pub bundle_identifier: Option<String>,
    pub bundle_url: Option<String>,
}

impl From<RunningApplication> for RunningAppInfo {
    fn from(running_application: RunningApplication) -> Self {
        unsafe {
            let pid = running_application.process_identifier();
            let localized_name = running_application.localized_name();
            let bundle_identifier = running_application.bundle_identifier();
            let bundle_url = running_application
                .bundle_url()
                .map(|url| url.absolute_string());

            RunningAppInfo {
                pid,
                localized_name,
                bundle_identifier,
                bundle_url,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppNotification {
    pub notification_type: NotificationType,
    pub app: RunningAppInfo,
}

impl AppNotification {
    /// Parses a notification from an `Id` to an `AppNotification`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences `notification`, which is a raw pointer.
    ///
    pub unsafe fn parse_notification(
        notification: Id,
        notification_type: NotificationType,
    ) -> Result<AppNotification, String> {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);

            let dict: Id = msg_send![notification, userInfo];

            {
                let dict = Dict::from_ptr(dict).unwrap();

                let application = dict
                    .get_id("NSWorkspaceApplicationKey")
                    .map(|ptr| RunningApplication::from_ptr(ptr).unwrap())
                    .unwrap();

                let notif = match notification_type {
                    NotificationType::Launched => {
                        // let application_path = dict.get_string("NSApplicationPath").unwrap();
                        // let bundle_identifier =
                        //     dict.get_string("NSApplicationBundleIdentifier").unwrap();
                        // let process_identifier =
                        //     dict.get_value::<i32>("NSApplicationProcessIdentifier");
                        // let pid = application.process_identifier();
                        // let application_name = dict.get_string("NSApplicationName").unwrap();

                        // let n = application_name.replace(".", "_").to_string();

                        // let user_dir = std::env::var("HOME").unwrap();

                        // let p = format!("{user_dir}/.hyperkey/app_icons/{}.tiff", n);
                        // dbg!(&p);

                        // dbg!(application.save_icon_to_file(&p));

                        /*
                        - [NSApplicationPath] (__NSCFString)
                        - [NSWorkspaceApplicationKey] (NSRunningApplication)
                        - [NSApplicationBundleIdentifier] (__NSCFString)
                        - [NSApplicationProcessSerialNumberLow] (__NSCFNumber)
                        - [NSApplicationProcessIdentifier] (__NSCFNumber)
                        - [NSApplicationProcessSerialNumberHigh] (__NSCFNumber)
                        - [NSApplicationName] (NSTaggedPointerString)
                        */
                        AppNotification {
                            notification_type,
                            app: application.into(),
                        }
                    }
                    NotificationType::Terminated => {
                        /*
                        - [NSApplicationPath] (__NSCFString)
                        - [NSWorkspaceApplicationKey] (NSRunningApplication)
                        - [NSApplicationBundleIdentifier] (__NSCFString)
                        - [NSWorkspaceExitStatusKey] (__NSCFNumber)
                        - [NSApplicationProcessIdentifier] (__NSCFNumber)
                        - [NSApplicationProcessSerialNumberLow] (__NSCFNumber)
                        - [NSApplicationProcessSerialNumberHigh] (__NSCFNumber)
                        - [NSApplicationName] (NSTaggedPointerString)
                        */
                        AppNotification {
                            notification_type,
                            app: application.into(),
                        }
                    }
                    NotificationType::Launching => {
                        // - [NSApplicationPath] (__NSCFString)
                        // - [NSWorkspaceApplicationKey] (NSRunningApplication)
                        // - [NSApplicationBundleIdentifier] (__NSCFString)
                        // - [NSApplicationProcessSerialNumberLow] (__NSCFNumber)
                        // - [NSApplicationProcessIdentifier] (__NSCFNumber)
                        // - [NSApplicationProcessSerialNumberHigh] (__NSCFNumber)
                        // - [NSApplicationName] (NSTaggedPointerString)
                        AppNotification {
                            notification_type,
                            app: application.into(),
                        }
                    }
                    NotificationType::Hidden => AppNotification {
                        notification_type,
                        app: application.into(),
                    },
                    NotificationType::Unhidden => AppNotification {
                        notification_type,
                        app: application.into(),
                    },
                    NotificationType::Activated => AppNotification {
                        notification_type,
                        app: application.into(),
                    },
                    NotificationType::Deactivated => AppNotification {
                        notification_type,
                        app: application.into(),
                    },
                };

                pool.drain();

                Ok(notif)
            }
        }
    }
}
