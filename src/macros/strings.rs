/// Generates C-like terminated string with the type of [`crate::types::StrPtr`]
/// ```
/// # use memflex::{cstr, terminated_array};
/// # unsafe {
/// assert_eq!(cstr!("123").as_str(), "123");
/// # }
/// ```
#[macro_export]
macro_rules! cstr {
    ( $($tt:tt)* ) => {
        $crate::types::StrPtr::from_ptr(unsafe { core::ptr::NonNull::new_unchecked(core::concat!($($tt)*, "\x00").as_ptr() as _) })
    }
}
