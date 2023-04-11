use crate::prelude::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(CocoaType)]
pub struct URL {
    ptr: Id,
}

#[allow(clippy::missing_safety_doc)]
impl URL {
    #[cocoa_instance_property(absoluteString)]
    pub unsafe fn absolute_string(&self) -> String {}

    #[cocoa_instance_property(absoluteURL)]
    pub unsafe fn absolute_url(&self) -> URL {}

    #[cocoa_instance_property(baseURL)]
    pub unsafe fn base_url(&self) -> Option<URL> {}

    #[cocoa_instance_property(fragment)]
    pub unsafe fn fragment(&self) -> Option<String> {}

    #[cocoa_instance_property(host)]
    pub unsafe fn host(&self) -> Option<String> {}

    #[cocoa_instance_property(lastPathComponent)]
    pub unsafe fn last_path_component(&self) -> String {}

    #[cocoa_instance_property(path)]
    pub unsafe fn path(&self) -> String {}

    // #[cocoa_instance_property(pathComponents)]
    // pub unsafe fn path_components(&self) -> Vec<String> {}

    #[cocoa_instance_property(pathExtension)]
    pub unsafe fn path_extension(&self) -> String {}

    // #[cocoa_instance_property(port)]
    // pub unsafe fn port(&self) -> Option<usize> {}

    #[cocoa_instance_property(query)]
    pub unsafe fn query(&self) -> Option<String> {}

    #[cocoa_instance_property(relativePath)]
    pub unsafe fn relative_path(&self) -> String {}

    #[cocoa_instance_property(relativeString)]
    pub unsafe fn relative_string(&self) -> String {}

    #[cocoa_instance_property(scheme)]
    pub unsafe fn scheme(&self) -> Option<String> {}

    #[cocoa_instance_property(standardized)]
    pub unsafe fn standardized(&self) -> URL {}

    #[cocoa_instance_property(standardizedFileURL)]
    pub unsafe fn standardized_file_url(&self) -> URL {}

    #[cocoa_instance_property(user)]
    pub unsafe fn user(&self) -> Option<String> {}

    #[cocoa_instance_property(password)]
    pub unsafe fn password(&self) -> Option<String> {}
}
