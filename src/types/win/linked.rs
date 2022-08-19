/// Single link of doubly linked list.
#[repr(C)]
pub struct ListEntry<T> {
    /// Next link
    pub next: *mut ListEntry<T>,
    /// Prev link
    pub prev: *mut ListEntry<T>,
}

impl<T> Clone for ListEntry<T> {
    fn clone(&self) -> Self {
        Self { next: self.next, prev: self.prev }
    }
}
impl<T> Copy for ListEntry<T> { }

/// Iterator over links of doubly linked list.
pub struct DoublyLinkedListIter<'a, T, const F: usize> {
    head: &'a ListEntry<T>,
    current: ListEntry<T>,
    start: bool
}

impl<'a, T, const F: usize> DoublyLinkedListIter<'a, T, F> {
    /// Creates new iterator over doubly linked list.
    pub fn new(head: &'a ListEntry<T>) -> Self {
        Self {
            head,
            current: *head,
            start: false,
        }
    }
}

impl<'a, T, const F: usize> Iterator for DoublyLinkedListIter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.start {
                if self.current.next as usize == self.head as *const _ as usize {
                    None
                } else {
                    let next = self.current.next.read();
                    let item = self.current.next
                        .cast::<u8>()
                        .sub(F)
                        .cast::<T>()
                        .read();
                    self.current = next;
                    Some(item)
                }
            } else {
                self.start = true;
                self.current = self.head.next.read();
                Some(self.head.next
                    .cast::<u8>()
                    .sub(F)
                    .cast::<T>()
                    .read())
            }
        }
    }
}