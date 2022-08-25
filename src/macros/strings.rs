/// Generates C-like terminated string with the type of [`crate::types::TStr`]
/// ```
/// # use memflex::{tstr, terminated_array};
/// # unsafe {
/// assert_eq!(tstr!("123").as_str(), "123");
/// # }
/// ```
#[macro_export]
macro_rules! tstr {
    ( $($tt:tt)* ) => {
        $crate::types::TStr::from_ptr(unsafe { core::ptr::NonNull::new_unchecked(core::concat!($($tt)*, "\x00").as_ptr() as _) })
    }
}
