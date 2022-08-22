use core::{marker::PhantomData, mem::transmute, ops::Deref};
use crate::cell::StaticCell;

/// Creates a function defenition with explicitly defined offset from module.
/// ```
/// # use memflex::ResolveBy;
/// fn get_address_in_module<const N: usize>(_: ResolveBy<N>) -> usize {
///     todo!()
/// }
/// 
/// memflex::function! {
///     extern fn CALLE(i32, i32) -> i32 = "file.exe"#0x1122;
/// 
///     extern fn OTHER_FN(f32, bool) -> u64 = (get_address_in_module)"file.exe"#0x1122;
/// 
///     // Resolve by signature
///     extern fn SIG_FN() -> u64 = "file.exe"%"4A 5B 3C AA 14";
/// }
/// ```
#[macro_export]
macro_rules! function {
    (
        $(
            $(extern $($abi:literal)?)? fn $fname:ident( $($atype:ty),* ) $(-> $ret:ty)? = $( ($resolver:ident) )? $modname:literal $sep:tt $offset:expr;
        )*
    ) => {
        $(
            static $fname: $crate::Function< $(extern $($abi)?)? fn($($atype),*) $(-> $ret)?> = $crate::Function::new(
                || unsafe {
                    ($crate::__resolver!( $($resolver)? ))( $crate::__resolve_by!($sep $modname, $offset ) )
                }
            );
        )*
    };
}

#[doc(hidden)]
pub struct Function<F> {
    cell: StaticCell<usize, fn() -> usize>,
    _ph: PhantomData<F>,
}

impl<F> Function<F> {
    #[doc(hidden)]
    pub const fn new(init: fn() -> usize) -> Self {
        Self {
            _ph: PhantomData,
            cell: StaticCell::new(init),
        }
    }
}

impl<F> Deref for Function<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.cell.value()) }
    }
}
