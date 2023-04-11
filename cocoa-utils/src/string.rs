use crate::prelude::*;

/// Rust wrapper around an `NSString` with helper methods.
#[allow(non_camel_case_types)]
#[derive(CocoaType)]
pub struct NS_String {
    ptr: Id,
}

const UTF8_ENCODING: NS_uint = 4;

impl NS_String {
    /// Allocates a new `NS_String` from a `&str`.
    pub fn from(content: &str) -> Self {
        let ptr: Id = unsafe {
            let string: Id = msg_send![class!(NSString), alloc];
            msg_send![string, initWithBytes:content.as_ptr()
                                     length:content.len()
                                   encoding:UTF8_ENCODING]
        };

        NS_String { ptr }
    }
}

impl ToString for NS_String {
    /// Creates a new `String` from the `NS_String`.
    fn to_string(&self) -> String {
        let c_str: *const libc::c_char = unsafe {
            let c_str: *const libc::c_char = msg_send![self.ptr, UTF8String];
            c_str
        };
        let c_str = unsafe { std::ffi::CStr::from_ptr(c_str) };
        c_str.to_string_lossy().into_owned()
    }
}
