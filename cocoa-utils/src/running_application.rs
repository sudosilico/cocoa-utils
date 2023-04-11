use cocoa::{appkit::NSImage, base::YES, foundation::NSData};

use crate::prelude::*;

#[derive(CocoaType)]
pub struct RunningApplication {
    ptr: Id,
}

#[allow(clippy::missing_safety_doc)]
impl RunningApplication {
    #[cocoa_type_property(NSRunningApplication, currentApplication)]
    pub unsafe fn current_application() -> RunningApplication {}

    #[cocoa_instance_property(active)]
    pub unsafe fn active(&self) -> bool {}

    #[cocoa_instance_property(hidden)]
    pub unsafe fn hidden(&self) -> bool {}

    #[cocoa_instance_property(localizedName)]
    pub unsafe fn localized_name(&self) -> Option<String> {}

    #[cocoa_instance_property(bundleIdentifier)]
    pub unsafe fn bundle_identifier(&self) -> Option<String> {}

    #[cocoa_instance_property(bundleURL)]
    pub unsafe fn bundle_url(&self) -> Option<URL> {}

    #[cocoa_instance_property(executableArchitecture)]
    pub unsafe fn executable_architecture(&self) -> isize {}

    #[cocoa_instance_property(executableURL)]
    pub unsafe fn executable_url(&self) -> Option<URL> {}

    #[cocoa_instance_property(isFinishedLaunching)]
    pub unsafe fn is_finished_launching(&self) -> bool {}

    #[cocoa_instance_property(processIdentifier)]
    pub unsafe fn process_identifier(&self) -> usize {}

    #[cocoa_instance_property(ownsMenuBar)]
    pub unsafe fn owns_menu_bar(&self) -> bool {}

    #[cocoa_instance_property(icon)]
    pub unsafe fn icon(&self) -> Id {}

    pub unsafe fn save_icon_to_file(&self, path: &str) -> bool {
        if std::path::Path::new(path).exists() {
            return false;
        }

        let path = NS_String::from(path);
        let icon = self.icon();
        let data = icon.TIFFRepresentation();

        data.writeToFile_atomically_(path.ptr(), YES) == YES
    }
}
