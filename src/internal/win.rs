use crate::{Matcher, DynPattern};

/// Searches for module's base address by its name.
/// # Behavior
/// Function iteraters over ldr searches for module entry (ascii case insensetive).
pub fn find_module_by_name(mod_name: &str) -> Option<crate::types::ModuleBasicInfo> {
    use crate::types::{ModuleBasicInfo, win::Teb};

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
pub fn find_pattern_in_module(
    pat: impl Matcher,
    mod_name: &str,
) -> Option<impl Iterator<Item = *const u8>> {
    let module = find_module_by_name(mod_name)?;
    unsafe { Some(crate::find_pattern(pat, module.base, module.size)) }
}

/// Creates a pattern for `target`, making sure there there are no other exact matches in the specified module.
pub fn create_pattern_in_module(
    target: *const u8,
    module_name: &str
) -> Option<DynPattern> {
    let module = find_module_by_name(module_name)?;
    unsafe { crate::create_pattern(target, module.base, module.size) }
}

/// Returns an iterator over all modules in the current process.
/// # Panics
/// If any module's name or path contain invalid UTF-16 sequence
#[cfg(feature = "alloc")]
pub fn modules() -> impl Iterator<Item = crate::types::ModuleAdvancedInfo> {
    use crate::types::{ModuleAdvancedInfo, win::Teb};

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
/// Can return `None` if the module was manually mapped and not linked in ldr.
#[cfg(feature = "alloc")]
pub fn current_module() -> Option<crate::types::ModuleAdvancedInfo> {
    let mut rip: usize;
    unsafe {
        core::arch::asm!("lea {}, [rip]", out(reg) rip);

        modules().find(|m| rip < m.base as usize + m.size && rip > m.base as usize)
    }
}
