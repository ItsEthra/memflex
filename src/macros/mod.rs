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
mod bitstruct;
pub use bitstruct::*;

pub(crate) mod cell;

/// How to resolve static offsets
pub enum ResolveBy<const N: usize> {
    /// By module name and offset
    NameOffset {
        /// Module name
        module_name: &'static str,
        /// Offset
        offset: usize,
    },
    /// By module name and pattern
    IdaPattern {
        /// Module name
        module_name: &'static str,
        /// Ida pattern
        pattern: crate::Pattern<N>,
    },
}

#[macro_export]
#[doc(hidden)]
macro_rules! __resolve_by {
    (# $module:literal, $second:literal) => {
        $crate::ResolveBy::<0>::NameOffset {
            module_name: $module,
            offset: $second,
        }
    };
    (% $module:literal, $second:literal) => {
        $crate::ResolveBy::IdaPattern {
            module_name: $module,
            pattern: $crate::ida_pat!($second),
        }
    };
}

#[doc(hidden)]
#[cfg(all(any(windows, unix), feature = "std"))]
pub fn __default_resolver<const N: usize>(res: ResolveBy<N>) -> usize {
    use crate::internal::{find_module_by_name, find_pattern_in_module};

    match res {
        ResolveBy::NameOffset {
            module_name,
            offset,
        } => {
            find_module_by_name(module_name)
                .expect("Module not found")
                .base as usize
                + offset
        }
        ResolveBy::IdaPattern {
            module_name,
            pattern,
        } => find_pattern_in_module(pattern, module_name)
            .unwrap()
            .next()
            .unwrap() as usize,
    }
}

#[doc(hidden)]
#[cfg(not(any(unix, windows)))]
pub fn __default_resolver<const N: usize>(res: ResolveBy<N>) -> usize {
    unimplemented!()
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolver {
    () => {
        $crate::__default_resolver
    };
    ($item:path) => {
        $item
    };
}

/// Gets the size in bytes of the type or the variable
/// ```
/// # use memflex::sizeof;
/// assert_eq!(sizeof!(i32), 4);
///
/// let var = 5_u64;
/// assert_eq!(sizeof!(@var), 8);
/// ```
#[macro_export]
macro_rules! sizeof {
    ($ty:ty) => {
        core::mem::size_of::<$ty>()
    };
    (@ $var:ident) => {
        core::mem::size_of_val(&$var)
    };
}
