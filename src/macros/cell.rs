use core::cell::UnsafeCell;

enum CellState<T> {
    Pending(fn() -> T),
    Ready(T),
}

pub(crate) struct StaticCell<T> {
    state: UnsafeCell<CellState<T>>,
}

impl<T> StaticCell<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            state: UnsafeCell::new(CellState::Pending(init)),
        }
    }

    #[inline]
    pub fn value(&self) -> &T {
        unsafe {
            match self.state.get().as_ref().unwrap() {
                CellState::Pending(ref init) => {
                    let value = init();
                    *self.state.get() = CellState::Ready(value);

                    self.value()
                }
                CellState::Ready(ref value) => value,
            }
        }
    }
}

unsafe impl<T> Sync for StaticCell<T> {}

#[test]
fn test_static_cell() {
    static CELL: StaticCell<i32> = StaticCell::new(move || 15);

    assert_eq!(*CELL.value(), 15);
}
