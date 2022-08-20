mod interface;
pub use interface::*;
mod makestruct;
pub use makestruct::*;
mod global;
pub use global::*;
mod strings;
pub use strings::*;
mod function;
pub use function::*;

pub(crate) mod cell;

#[doc(hidden)]
#[cfg(windows)]
pub fn __default_resolver(mod_name: &str, offset: usize) -> usize {
    use crate::internal::find_module_by_name;

    find_module_by_name(mod_name)
        .expect("Module not found")
        .base as usize
        + offset
}

#[doc(hidden)]
#[cfg(not(windows))]
pub fn __default_resolver(mod_name: &str, offset: usize) -> usize {
    todo!()
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolver {
    () => {
        $crate::__default_resolver
    };
    ($($tt:tt)*) => {
        $($tt)*
    }
}
