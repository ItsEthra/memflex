use core::{
    marker::PhantomData,
    ops::Deref,
    ptr::addr_of,
    sync::atomic::{AtomicUsize, Ordering},
};

/// Creates a function defenition with explicitly defined offset from module or signature.
/// ```
/// # use memflex::ResolveBy;
/// fn get_address_in_module<const N: usize>(_: ResolveBy<N>) -> usize {
///     todo!()
/// }
///
/// memflex::function! {
///     extern fn CALLE(i32, i32) -> i32 = "file.exe"#0x1122;
///
///     // Use `get_address_in_module` function to get the address duh
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

/// Function that resolve its address on the first access.
pub struct Function<F> {
    address: AtomicUsize,
    init: fn() -> usize,
    _ph: PhantomData<F>,
}

impl<F> Function<F> {
    #[doc(hidden)]
    pub const fn new(init: fn() -> usize) -> Self {
        Self {
            address: AtomicUsize::new(0),
            init,
            _ph: PhantomData,
        }
    }

    /// Returns function address
    pub fn address(&self) -> usize {
        let mut address = self.address.load(Ordering::Acquire);
        if address == 0 {
            address = (self.init)();
            self.address.store(address, Ordering::Release);
        }

        address
    }

    /// Force inits function
    pub fn force(&self) {
        _ = self.address();
    }
}

impl<F> Deref for Function<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        self.force();

        unsafe {
            addr_of!(self.address)
                .cast::<Self::Target>()
                .as_ref()
                .unwrap()
        }
    }
}
