use crate::pattern::Pattern;
use core::{ops::RangeInclusive, slice::from_raw_parts};

/// # Safety
/// * `first` - a valid pointer.
/// * `last` - is contained within `usize::MAX` bytes away from `first`.
/// ```
/// # use memflex::internal::terminated_array;
///
/// let items = b"123\x00";
/// # unsafe {
/// assert_eq!(terminated_array(items.as_ptr(), 0), &[b'1', b'2', b'3'])
/// # }
/// ```
#[inline]
pub unsafe fn terminated_array<'a, T: PartialEq>(mut first: *const T, last: T) -> &'a [T] {
    let mut len = 0;
    while *first != last {
        len += 1;
        first = first.add(1);
    }

    core::slice::from_raw_parts(first.sub(len), len)
}

/// # Safety
/// * `first` - a valid pointer.
/// * `last` - is contained within `usize::MAX` bytes away from `first`.
/// ```
/// # use memflex::internal::terminated_array;
///
/// let items = b"123\x00";
/// # unsafe {
/// assert_eq!(terminated_array(items.as_ptr(), 0), &[b'1', b'2', b'3'])
/// # }
/// ```
#[inline]
pub unsafe fn terminated_array_mut<'a, T: PartialEq>(mut first: *mut T, last: T) -> &'a mut [T] {
    let mut len = 0;
    while *first != last {
        len += 1;
        first = first.add(1);
    }

    core::slice::from_raw_parts_mut(first.sub(len), len)
}

/// Searches for a pattern internally by start address and search length
/// # Safety
/// * `start` is a valid pointer and can be read
/// * Memory from `start` to `start + len` (inclusive) can be read
#[inline]
pub unsafe fn find_pattern<const N: usize>(
    pat: Pattern<N>,
    start: *const u8,
    len: usize,
) -> impl Iterator<Item = *const u8> {
    from_raw_parts::<u8>(start, len)
        .windows(N)
        .enumerate()
        .filter_map(move |(i, bytes)| {
            if pat.matches(bytes) {
                Some(start.add(i))
            } else {
                None
            }
        })
}

/// Searches for a pattern internally in a given range
/// # Safety
/// * Range represents a chunk of memory that can be read.
#[inline]
pub unsafe fn find_pattern_range<const N: usize>(
    pat: Pattern<N>,
    range: RangeInclusive<usize>,
) -> impl Iterator<Item = *const u8> {
    find_pattern(pat, *range.start() as _, *range.end() - *range.start())
}

/// Searches for module's base address by its name.
/// # Behavior
/// Function iteraters over ldr searches for module entry (ascii case insensetive).
#[cfg(windows)]
pub fn find_module_by_name(mod_name: &str) -> Option<crate::types::ModuleBasicInfo> {
    use crate::types::{ModuleBasicInfo, Teb};

    Teb::current()
        .peb
        .ldr
        .iter()
        .filter(|e| e.base_dll_name.len() == mod_name.len())
        .find_map(|e| {
            if unsafe {
                e.base_dll_name
                    .utf16()
                    .zip(mod_name.chars())
                    .all(|(a, b)| a.eq_ignore_ascii_case(&b))
            } {
                Some(ModuleBasicInfo {
                    size: e.image_size as usize,
                    base: e.dll_base,
                })
            } else {
                None
            }
        })
}

/// Searches for a pattern in the specified module.
#[cfg(windows)]
pub fn find_pattern_in_module<const N: usize>(
    pat: Pattern<N>,
    mod_name: &str,
) -> Option<*const u8> {
    let module = find_module_by_name(mod_name)?;

    unsafe { find_pattern(pat, module.base, module.size).next() }
}

/// Returns an iterator over all modules in the current process.
/// # Panics
/// If any module's name or path contain invalid UTF-16 sequence
#[cfg(all(windows, feature = "alloc"))]
pub fn modules() -> impl Iterator<Item = crate::types::ModuleAdvancedInfo> {
    use crate::types::{ModuleAdvancedInfo, Teb};

    Teb::current().peb.ldr.iter().map(|e| unsafe {
        ModuleAdvancedInfo {
            base: e.dll_base,
            size: e.image_size as usize,
            name: e.base_dll_name.to_string().unwrap(),
            path: e.full_dll_name.to_string().unwrap(),
        }
    })
}

/// Returns an information about current module
/// # Behavior
/// Looks up module by looking up RIP register.
/// Can return `None` if module was manually mapped and not linked in ldr.
pub fn current_module() -> Option<crate::types::ModuleAdvancedInfo> {
    let mut rip: usize;
    unsafe {
        core::arch::asm!("lea {}, [rip]", out(reg) rip);
        
        modules().find(|m| rip < m.base as usize + m.size && rip > m.base as usize)
    }
}