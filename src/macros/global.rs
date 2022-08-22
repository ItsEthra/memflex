use core::ops::{Deref, DerefMut};
use core::marker::PhantomData;
use crate::cell::StaticCell;
use core::mem::transmute;

#[doc(hidden)]
pub struct Global<T> {
    cell: StaticCell<usize, fn() -> usize>,
    _ph: PhantomData<T>,
}

impl<T> Global<T> {
    #[doc(hidden)]
    pub const fn new(
        init: fn() -> usize
    ) -> Self {
        Self {
            _ph: PhantomData,
            cell: StaticCell::new(init),
        }
    }
}

impl<T> Deref for Global<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(*self.cell.value()) }
    }
}

impl<T> DerefMut for Global<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(*self.cell.value()) }
    }
}

/// Declares global variables with fixed offset from module
/// ```
/// # use memflex::ResolveBy;
/// fn get_address_in_module<const N: usize>(_: ResolveBy<N>) -> usize {
///     todo!()
/// }
///
/// memflex::global! {
///     // Uses default ldr resolver
///     pub static MY_GLOBAL: i32 = "app.exe"#0xAABB;
///
///     // Or use another function to get offset
///     pub static HEALTH: f32 = (get_address_in_module)"app.exe"#0xFFEE;
/// }
/// ```
#[macro_export]
macro_rules! global {
    {
        $(
            $vs:vis static $gname:ident: $gtype:ty = $( ($resolver:ident) )? $module:literal $sep:tt $offset:expr;
        )*
    } => {
        $(
            $vs static $gname: $crate::Global<$gtype> = $crate::Global::new(
                || unsafe { ($crate::__resolver!( $($resolver)? ))( $crate::__resolve_by!($sep $module, $offset) ) }
            );
        )*
    };
}
