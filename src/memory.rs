use core::{ops::RangeInclusive, slice::from_raw_parts};
use crate::{Matcher, DynPattern};

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
pub unsafe fn resolve_multilevel<T>(mut base: *const usize, offsets: &[usize]) -> *const T {
    offsets.iter()
        .for_each(|&o| {
            base = base.cast::<u8>()
                .add(o)
                .cast::<usize>()
                .read() as _;
        });

    base as _
}

/// Resolves mutable multilevel pointer.
pub unsafe fn resolve_multilevel_mut<T>(mut base: *mut usize, offsets: &[usize]) -> *mut T {
    offsets.iter()
        .for_each(|&o| {
            base = base.cast::<u8>()
                .add(o)
                .cast::<usize>()
                .read() as _;
        });

    base as _
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
        .windows(pat.size())
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

/// Creates a pattern for `target` address, making sure there are no exact matches in range from `start` to `start + len`.
/// If `max` is set, function will abort if failed to find pattern in less than `max` bytes.
pub unsafe fn create_pattern(
    target: *const u8,
    start: *const u8,
    len: usize,
    max: Option<usize>,
) -> Option<DynPattern>
{
    let mut size = 3;
    let mut offset = 0;
    let span = from_raw_parts(start, len);
    
    loop {
        let pat = from_raw_parts(target, size);
        if let Some((i, _)) = span
            .windows(size)
            .enumerate()
            .skip(offset)
            .find(|(i, seq)| pat.matches(seq) && start.add(*i) != target)
        {
            size += 1;
            offset = i;

            if let Some(max) = max && size > max {
                return None;
            }

            continue;
        }

        break Some(pat.into());
    }
}

/// Creates a pattern for `target` address, making sure there are no other exact matches in `range`.
/// If `max` is set, function will abort if failed to find pattern in less than `max` bytes.
pub unsafe fn create_pattern_range(
    target: *const u8,
    range: RangeInclusive<usize>,
    max: Option<usize>,
) -> Option<DynPattern>
{
    create_pattern(target, *range.start() as _, *range.end() - *range.start(), max)
}