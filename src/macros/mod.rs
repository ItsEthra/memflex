mod interface;
pub use interface::*;
mod makestruct;
pub use makestruct::*;
mod global;
pub use global::*;
mod function;
pub use function::*;
mod bitstruct;
pub use bitstruct::*;

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
#[cfg(all(windows, feature = "std"))]
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
#[cfg(not(windows))]
pub fn __default_resolver<const N: usize>(_: ResolveBy<N>) -> usize {
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
/// # use memflex::size_of;
/// assert_eq!(size_of!(i32), 4);
///
/// let var = 5_u64;
/// assert_eq!(size_of!(@var), 8);
/// ```
#[macro_export]
macro_rules! size_of {
    ($ty:ty) => {
        core::mem::size_of::<$ty>()
    };
    (@ $var:ident) => {
        core::mem::size_of_val(&$var)
    };
}

/// Gets the offset of the field.
/// ```
/// # use memflex::offset_of;
/// #[repr(C)]
/// struct Foo {
///     a: u32,
///     b: f32,
///     value: usize
/// }
///
/// let offset = offset_of!(Foo, value);
/// assert_eq!(offset, 8);
///
/// const CONSTANT: usize = offset_of!(Foo, value);
/// assert_eq!(CONSTANT, 8);
/// ```
#[macro_export]
macro_rules! offset_of {
    ($target:ty, $($field:tt).* $(,)?) => {{
        let base = core::mem::MaybeUninit::<$target>::uninit();
        let base_ptr = base.as_ptr();

        #[allow(unused_unsafe)]
        unsafe {
            let field_ptr = core::ptr::addr_of!( (*base_ptr) $(.$field)* ).cast::<u8>();
            field_ptr.offset_from(base_ptr.cast::<u8>()) as usize
        }
    }};
}

/// Asserts size of the type
/// ```
/// # use memflex::assert_size;
/// #[repr(C)]
/// struct Foo {
///     a: u32,
///     b: i32
/// }
/// // Doesn't compile
/// // assert_size!(Foo, 4);
///
/// // Works fine
/// assert_size!(Foo, 8);
/// ```
#[macro_export]
macro_rules! assert_size {
    ($target:ty, $size:expr) => {
        const _: () = if core::mem::size_of::<$target>() != $size {
            panic!(concat!(
                "Size assertion failed! sizeof(",
                stringify!($target),
                ") != ",
                stringify!($size)
            ))
        };
    };
}

/// Asserts offset of the fields
/// ```
/// # use memflex::assert_offset;
/// #[repr(C)]
/// struct Foo {
///     a: u32,
///     b: i32
/// }
///
/// assert_offset!(Foo, a, 0, b, 4);
/// ```
#[macro_export]
macro_rules! assert_offset {
    ($target:ty, $( $($field:ident).*, $offset:expr),* $(,)?) => {
        $(
            const _: () = if memflex::offset_of!($target, $($field).*) != $offset {
                panic!(concat!(
                    "Offset assertion failed! offset_of!(",
                    stringify!($target),
                    ", ",
                    stringify!( $($field).* ),
                    ") != ",
                    stringify!($offset)
                ))
            };
        )*
    }
}
