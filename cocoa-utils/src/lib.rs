mod app_watcher;
mod cocoa_type;
mod dict;
mod notification_center;
mod notification_type;
mod object;
mod running_application;
mod string;
mod url;
mod workspace;

pub mod prelude;

pub use app_watcher::*;
pub use dict::*;
pub use notification_center::*;
pub use notification_type::*;
pub use running_application::*;
pub use string::*;
pub use url::*;
pub use workspace::*;

#[link(name = "Foundation", kind = "framework")]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[macro_use]
extern crate objc;

#[allow(non_camel_case_types)]
#[cfg(target_pointer_width = "32")]
pub type NS_uint = libc::c_uint;

#[allow(non_camel_case_types)]
#[cfg(target_pointer_width = "64")]
pub type NS_uint = libc::c_ulong;

#[allow(non_camel_case_types)]
#[cfg(target_pointer_width = "32")]
pub type NS_int = libc::c_int;

#[allow(non_camel_case_types)]
#[cfg(target_pointer_width = "64")]
pub type NS_int = libc::c_long;
