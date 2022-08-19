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

    /// Resolves global's offset
    /// # Safety
    /// * Refer to the safety of the resolver function
    #[inline]
    pub unsafe fn resolve(&self) -> usize {
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

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.resolve()) }
    }
}

impl<T> DerefMut for Global<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.resolve()) }
    }
}

#[doc(hidden)]
pub fn __default_resolver(_: &str, _: usize) -> usize {
    todo!()
}
