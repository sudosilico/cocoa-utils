use crate::prelude::*;

#[derive(CocoaType)]
pub struct Workspace {
    ptr: Id,
}

#[allow(clippy::missing_safety_doc)]
impl Workspace {
    #[cocoa_type_property(NSWorkspace, sharedWorkspace)]
    pub unsafe fn shared_workspace() -> Workspace {}

    #[cocoa_instance_property(notificationCenter)]
    pub unsafe fn notification_center(&self) -> NotificationCenter {}

    #[cocoa_instance_property(frontmostApplication)]
    pub unsafe fn frontmost_application(&self) -> RunningApplication {}
}
