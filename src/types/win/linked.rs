/// Single link of doubly linked list
#[repr(C)]
pub struct ListEntry<T> {
    /// Next link
    pub next: *mut ListEntry<T>,
    /// Prev link
    pub prev: *mut ListEntry<T>,
}
