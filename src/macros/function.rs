use core::marker::PhantomData;
use core::mem::transmute;
use core::ops::Deref;
use core::sync::atomic::{AtomicBool, Ordering};

/// Creates a function defenition with explicitly defined offset from module.
/// ```
/// memflex::function! {
///     extern "C" fn CALLE(i32, i32) -> i32 = "file.exe"#0x1122;
/// }
/// ```
#[macro_export]
macro_rules! function {
    (
        $(
            $(extern $($abi:literal)?)? fn $fname:ident( $($atype:ty),* ) $(-> $ret:ty)? = $($resolver:ident)? $modname:literal $sep:tt $offset:expr;
        )*
    ) => {
        $(
            static $fname: $crate::Function< $(extern $($abi)?)? fn($($atype),*) $(-> $ret)?> = $crate::Function::new(
                $crate::__resolver!( $($resolver)? ),
                $modname,
                $offset
            );
        )*
    };
}

/// Global variable that points to memory location.
pub struct Function<T> {
    resolver: unsafe fn(&str, usize) -> usize,
    module: &'static str,
    offset: usize,
    address: usize,
    resolved: AtomicBool,

    _ph: PhantomData<T>,
}

impl<T> Function<T> {
    /// Creates new global that will resolve its address by module and offset on first access.
    pub const fn new(
        resolver: unsafe fn(&str, usize) -> usize,
        module: &'static str,
        offset: usize,
    ) -> Self {
        Self {
            resolved: AtomicBool::new(false),
            _ph: PhantomData,
            address: 0,
            resolver,
            module,
            offset,
        }
    }

    /// Resolves global's offset
    /// # Safety
    /// * Refer to the safety of the resolver function
    #[inline]
    pub unsafe fn resolve(&self) -> &usize {
        if !self.resolved.load(Ordering::Relaxed) {
            self.resolved.store(true, Ordering::Relaxed);
            (&mut *(self as *const Self as *mut Self)).address =
                (self.resolver)(self.module, self.offset);
        }

        &self.address
    }
}

impl<T> Deref for Function<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.resolve()) }
    }
}
