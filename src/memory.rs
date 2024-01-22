use crate::Matcher;
use core::{ops::RangeInclusive, slice::from_raw_parts};

/// Creates an inmmutable slice from terminated array.
/// # Safety
/// * `first` - a valid pointer.
/// * `last` - is contained within `usize::MAX` bytes away from `first`.
/// ```
/// # use memflex::terminated_array;
/// let items = b"123\x00";
/// # unsafe {
/// assert_eq!(terminated_array(items.as_ptr(), 0), &[b'1', b'2', b'3'])
/// # }
/// ```
#[inline]
pub unsafe fn terminated_array<'a, T: PartialEq>(mut first: *const T, last: T) -> &'a [T] {
    assert!(!first.is_null());

    let mut len = 0;
    while *first != last {
        len += 1;
        first = first.add(1);
    }

    core::slice::from_raw_parts(first.sub(len), len)
}

/// Creates a mutable slice from terminated array.
/// # Safety
/// * `first` - a valid pointer.
/// * `last` - is contained within `usize::MAX` bytes away from `first`.
/// ```
/// # use memflex::terminated_array;
/// let items = b"123\x00";
/// # unsafe {
/// assert_eq!(terminated_array(items.as_ptr(), 0), &[b'1', b'2', b'3'])
/// # }
/// ```
#[inline]
pub unsafe fn terminated_array_mut<'a, T: PartialEq>(mut first: *mut T, last: T) -> &'a mut [T] {
    assert!(!first.is_null());

    let mut len = 0;
    while *first != last {
        len += 1;
        first = first.add(1);
    }

    core::slice::from_raw_parts_mut(first.sub(len), len)
}

/// Resolves immutable multilevel pointer.
/// # Safety
/// * All offsets must lead to valid memory addresses.
#[inline]
pub unsafe fn resolve_multilevel<T>(mut base: *const u8, offsets: &[usize]) -> *const T {
    offsets.iter().enumerate().for_each(|(i, &o)| {
        if i != offsets.len() - 1 {
            base = base.add(o).cast::<*const u8>().read();
        } else {
            base = base.add(o);
        }
    });

    base.cast()
}

/// Resolves mutable multilevel pointer.
/// # Safety
/// * All offsets must lead to valid memory addresses.
#[inline]
pub unsafe fn resolve_multilevel_mut<T>(mut base: *mut u8, offsets: &[usize]) -> *mut T {
    offsets.iter().enumerate().for_each(|(i, &o)| {
        if i != offsets.len() - 1 {
            base = base.add(o).cast::<*mut u8>().read();
        } else {
            base = base.add(o);
        }
    });

    base.cast()
}

/// Searches for a pattern internally by start address and search length.
/// # Safety
/// * `start` is a valid pointer and can be read
/// * Memory from `start` to `start + len` (inclusive) can be read
#[inline]
pub unsafe fn find_pattern(
    pat: impl Matcher,
    start: *const u8,
    len: usize,
) -> impl Iterator<Item = *const u8> {
    assert!(!start.is_null());

    from_raw_parts::<u8>(start, len)
        .windows(pat.len())
        .enumerate()
        .filter_map(move |(i, bytes)| {
            if pat.matches(bytes) {
                Some(start.add(i))
            } else {
                None
            }
        })
}

/// Searches for a pattern internally in a given range.
/// # Safety
/// * Range represents a chunk of memory that can be read.
#[inline]
pub unsafe fn find_pattern_range(
    pat: impl Matcher,
    range: RangeInclusive<usize>,
) -> impl Iterator<Item = *const u8> {
    find_pattern(pat, *range.start() as _, *range.end() - *range.start())
}

#[cfg(test)]
mod tests {
    use crate::resolve_multilevel;
    use core::{mem::zeroed, ptr::addr_of};

    #[test]
    fn test_multilevel() {
        #[repr(C)]
        struct S1 {
            pad: [u8; 0x10],
            next: Box<S2>,
        }

        #[repr(C)]
        struct S2 {
            pad: [u8; 0x20],
            next: Box<S3>,
        }

        #[repr(C)]
        struct S3 {
            pad: [u8; 0x30],
            value: usize,
        }

        let s1 = unsafe {
            S1 {
                next: S2 {
                    next: S3 {
                        value: 1337,
                        pad: zeroed(),
                    }
                    .into(),
                    pad: zeroed(),
                }
                .into(),
                pad: zeroed(),
            }
        };

        unsafe {
            let ptr = resolve_multilevel::<usize>(addr_of!(s1).cast(), &[0x10, 0x20, 0x30]);
            assert_eq!(*ptr, 1337);
        }
    }
}
