/// Generates C-like terminated string with the type of `*const i8`
/// ```
/// # use memflex::{cstr, internal::terminated_array};
/// # unsafe {
/// assert_eq!(terminated_array(cstr!("123"), 0), &[b'1' as i8, b'2' as i8, b'3' as i8]);
/// # }
/// ```
#[macro_export]
macro_rules! cstr {
    ( $($tt:tt)* ) => {
        core::concat!( $($tt)*, "\x00" ).as_ptr() as *const i8
    }
}
