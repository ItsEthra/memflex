use crate::pattern::Pattern;

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
    unsafe { crate::find_pattern(pat, module.base, module.size).next() }
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
#[cfg(all(windows, feature = "alloc"))]
pub fn current_module() -> Option<crate::types::ModuleAdvancedInfo> {
    let mut rip: usize;
    unsafe {
        core::arch::asm!("lea {}, [rip]", out(reg) rip);
        
        modules().find(|m| rip < m.base as usize + m.size && rip > m.base as usize)
    }
}