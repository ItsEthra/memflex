use core::sync::atomic::{AtomicBool, Ordering};

pub(crate) struct StaticCell<V, F: FnOnce() -> V + 'static + Send + Sync = fn() -> V> {
    init: F,
    value: Option<V>,
    ready: AtomicBool,
}

impl<V, F: FnOnce() -> V + 'static + Send + Sync> StaticCell<V, F> {
    pub const fn new(init: F) -> Self {
        Self {
            init,
            value: None,
            ready: AtomicBool::new(false),
        }
    }

    #[allow(clippy::cast_ref_to_mut)]
    #[inline]
    pub fn init(&self) {
        unsafe {
            let this = &mut *(self as *const Self as *mut Self);
            let v = core::ptr::read(&this.init);
            this.value = Some(v());
        }
    }

    #[inline]
    pub fn value(&self) -> &V {
        if !self.ready.load(Ordering::SeqCst) {
            self.init();
            self.ready.store(true, Ordering::SeqCst);
        }

        self.value.as_ref().unwrap()
    }
}

#[test]
fn test_static_cell() {
    static CELL: StaticCell<i32> = StaticCell::new(move || 15);

    assert_eq!(*CELL.value(), 15);
}
