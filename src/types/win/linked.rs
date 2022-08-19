use core::ptr::NonNull;

/// Single link of doubly linked list.
#[repr(C)]
pub struct ListEntry<T> {
    /// Next link
    pub next: Option<NonNull<ListEntry<T>>>,
    /// Prev link
    pub prev: *mut Option<NonNull<ListEntry<T>>>,
}

impl<T> Clone for ListEntry<T> {
    fn clone(&self) -> Self {
        Self {
            next: self.next,
            prev: self.prev,
        }
    }
}
impl<T> Copy for ListEntry<T> {}

/// Iterator over links of doubly linked list.
pub struct DoublyLinkedListIter<'a, T> {
    head: &'a ListEntry<T>,
    current: ListEntry<T>,
    start: bool,
    offset: usize,
}

impl<'a, T> DoublyLinkedListIter<'a, T> {
    /// Creates new iterator over doubly linked list.
    pub fn new(head: &'a ListEntry<T>, offset: usize) -> Self {
        Self {
            head,
            current: *head,
            start: false,
            offset,
        }
    }
}

impl<'a, T> Iterator for DoublyLinkedListIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.start {
                if self.current.next?.as_ptr() as usize == self.head as *const _ as usize {
                    None
                } else {
                    let next = self.current.next?.as_ptr().read();
                    let item = self
                        .current
                        .next?
                        .as_ptr()
                        .cast::<u8>()
                        .sub(self.offset)
                        .cast::<T>()
                        .read();
                    self.current = next;
                    Some(item)
                }
            } else {
                self.start = true;
                self.current = self.head.next?.as_ptr().read();
                Some(
                    self.head
                        .next?
                        .as_ptr()
                        .cast::<u8>()
                        .sub(self.offset)
                        .cast::<T>()
                        .read(),
                )
            }
        }
    }
}

/// Handy macro to easily create iterator over doubly linked list
#[macro_export]
macro_rules! iter_list {
    ($head:expr, $entry:ty, $field:ident) => {
        $crate::types::DoublyLinkedListIter::new(
            $head,
            $crate::memoffset::offset_of!($entry, $field),
        )
    };
}
