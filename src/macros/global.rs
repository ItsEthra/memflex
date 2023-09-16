use core::marker::PhantomData;
use core::mem::transmute;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Global variable with explicitly defined offset.
pub struct Global<T> {
    address: AtomicUsize,
    init: fn() -> usize,
    _pd: PhantomData<T>,
}

impl<T> Global<T> {
    #[doc(hidden)]
    pub const fn new(init: fn() -> usize) -> Self {
        Self {
            address: AtomicUsize::new(0),
            init,
            _pd: PhantomData,
        }
    }

    /// Returns the address of the global.
    pub fn address(&self) -> usize {
        let mut value = self.address.load(Ordering::Acquire);
        if value == 0 {
            value = (self.init)();
            self.address.store(value, Ordering::Release);
        }

        value
    }

    /// Force resolves the address.
    pub fn force(&self) {
        _ = self.address();
    }
}

impl<T> Deref for Global<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let value = self.address();
        unsafe { transmute(value) }
    }
}

impl<T> DerefMut for Global<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value = self.address();
        unsafe { transmute(value) }
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
///     pub extern MY_GLOBAL: i32 = "app.exe"#0xAABB;
///
///     // Or use another function to get offset
///     pub extern HEALTH: f32 = (get_address_in_module)"app.exe"#0xFFEE;
/// }
/// ```
#[macro_export]
macro_rules! global {
    {
        $(
            $vs:vis extern $gname:ident: $gtype:ty = $( ($resolver:path) )? $module:literal $sep:tt $offset:expr;
        )*
    } => {
        $(
            $vs static $gname: $crate::Global<$gtype> = $crate::Global::new(
                || unsafe { ($crate::__resolver!( $($resolver)? ))( $crate::__resolve_by!($sep $module, $offset) ) }
            );
        )*
    };
}
