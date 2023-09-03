use core::{marker::PhantomData, ptr::NonNull};

/// Single link of doubly linked list.
#[repr(C)]
pub struct ListEntry<const OFFSET: usize, T> {
    next: Option<NonNull<Self>>,
    prev: Option<NonNull<Self>>,
    _ty: PhantomData<*mut T>,
}

impl<const OFFSET: usize, T> Clone for ListEntry<OFFSET, T> {
    fn clone(&self) -> Self {
        Self {
            _ty: PhantomData,
            next: self.next,
            prev: self.prev,
        }
    }
}
impl<const OFFSET: usize, T> Copy for ListEntry<OFFSET, T> {}

impl<const OFFSET: usize, T> ListEntry<OFFSET, T> {
    /// Returns null `ListEntry`.
    #[inline]
    pub fn null() -> Self {
        Self {
            next: None,
            prev: None,
            _ty: PhantomData,
        }
    }

    /// Returns pointer to the next entry.
    #[inline]
    pub fn next(&self) -> Option<NonNull<T>> {
        let next_entry = self.next?.as_ptr().cast::<u8>();
        NonNull::new((next_entry as usize - OFFSET) as *mut T)
    }

    /// Returns pointer to the previous entry.
    #[inline]
    pub fn prev(&self) -> Option<NonNull<T>> {
        let next_entry = self.prev?.as_ptr().cast::<u8>();
        NonNull::new((next_entry as usize - OFFSET) as *mut T)
    }

    /// Assuming `self` is a list head, iterate over immutable references to all items in the list.
    pub unsafe fn iter<'r>(&self) -> impl Iterator<Item = &'r T> {
        let last = self.prev.map(|v| v.as_ptr().cast_const());
        let mut current = last;
        core::iter::from_fn(move || unsafe {
            let r = (*current?).next()?.as_ref();
            current = Some((*current?).next?.as_ptr().cast_const());
            if current? as usize == last? as usize {
                current = None;
            }

            Some(r)
        })
    }

    /// Assuming `self` is a list head, iterate over mutable references to all items in the list.
    pub unsafe fn iter_mut<'r>(&mut self) -> impl Iterator<Item = &'r mut T> {
        let last = self.prev.map(|v| v.as_ptr());
        let mut current = last;
        core::iter::from_fn(move || unsafe {
            let r = (*current?).next()?.as_mut();
            current = Some((*current?).next?.as_ptr());
            if current? as usize == last? as usize {
                current = None;
            }

            Some(r)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ListEntry;
    use crate::assert_offset;
    use core::ptr::NonNull;

    #[repr(C)]
    struct Value {
        value: i32,
        next: ListEntry<8, Value>,
    }
    assert_offset!(Value, next, 8);

    #[test]
    fn test_list_iter() {
        unsafe {
            let mut a = Value {
                value: 1,
                next: ListEntry::null(),
            };
            let mut b = Value {
                value: 2,
                next: ListEntry::null(),
            };
            let mut c = Value {
                value: 3,
                next: ListEntry::null(),
            };

            a.next.prev = Some(NonNull::new_unchecked(&mut c.next));
            a.next.next = Some(NonNull::new_unchecked(&mut b.next));

            b.next.prev = Some(NonNull::new_unchecked(&mut a.next));
            b.next.next = Some(NonNull::new_unchecked(&mut c.next));

            c.next.prev = Some(NonNull::new_unchecked(&mut b.next));
            c.next.next = Some(NonNull::new_unchecked(&mut a.next));

            assert_eq!(
                &a.next.iter().map(|v| v.value).collect::<Vec<_>>(),
                &[1, 2, 3]
            );
        }
    }
}
