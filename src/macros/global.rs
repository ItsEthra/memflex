use core::marker::PhantomData;
use core::mem::transmute;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Global variable that points to memory location.
pub struct Global<T> {
    resolver: unsafe fn(&str, usize) -> usize,
    module: &'static str,
    address: AtomicUsize,
    offset: usize,

    _ph: PhantomData<T>,
}

impl<T> Global<T> {
    /// Creates new global that will resolve its address by module and offset on first access.
    pub const fn new(
        resolver: unsafe fn(&str, usize) -> usize,
        module: &'static str,
        offset: usize,
    ) -> Self {
        Self {
            address: AtomicUsize::new(0),
            _ph: PhantomData,
            resolver,
            module,
            offset,
        }
    }

    #[inline]
    unsafe fn resolve(&self) -> usize {
        let mut addr = self.address.load(Ordering::Relaxed);
        if addr == 0 {
            addr = (self.resolver)(self.module, self.offset);
            self.address.store(addr, Ordering::Relaxed);
        }

        addr
    }
}

impl<T> Deref for Global<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.resolve()) }
    }
}

impl<T> DerefMut for Global<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.resolve()) }
    }
}

/// Declares global variables with fixed offset from module
/// ```
/// fn get_module_by_address(module: &str, offset: usize) -> usize {
///     todo!()
/// }
///
/// memflex::global! {
///     // Uses default ldr resolver
///     pub static MY_GLOBAL: i32 = "app.exe"#0xAABB;
///
///     // Or use another function to get offset
///     pub static HEALTH: f32 = (get_module_by_address)"app.exe"#0xFFEE;
/// }
/// ```
#[macro_export]
macro_rules! global {
    {
        $(
            $vs:vis static $gname:ident: $gtype:ty = $( ($resolver:ident) )? $modname:literal $sep:tt $offset:expr;
        )*
    } => {
        $(
            $vs static $gname: $crate::Global<$gtype> = $crate::Global::new($crate::__resolver!( $($resolver)? ), $modname, $offset);
        )*
    };
}
