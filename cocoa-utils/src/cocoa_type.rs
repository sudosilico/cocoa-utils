use super::prelude::*;

pub trait CocoaType {
    /// Creates a new instance of the type from the given pointer.
    ///
    /// # Safety
    ///
    /// TODO: Document safety requirements.
    unsafe fn from_ptr(ptr: Id) -> Option<Self>
    where
        Self: Sized;

    /// Gets the raw pointer to the object, as an [`Id`] (which is a type alias to [`*mut objc::runtime::Object`]).
    ///
    /// # Safety
    ///
    /// TODO: Document safety requirements.
    unsafe fn ptr(&self) -> Id;

    /// Gets the class name for the type.
    fn class_name() -> &'static str;
}
