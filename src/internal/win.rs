use crate::{types::win::KeyCode, DynPattern, Matcher};

/// Searches for a module by its name.
/// # Behavior
/// Function iteraters over ldr searches for module entry (ascii case insensetive).
pub fn find_module_by_name(name: &str) -> Option<crate::types::ModuleBasicInfo> {
    use crate::types::{win::Teb, ModuleBasicInfo};

    Teb::current()
        .peb
        .ldr
        .iter()
        .filter(|e| e.base_dll_name.len() == module_name.len())
        .find_map(|e| {
            if unsafe {
                e.base_dll_name
                    .utf16()
                    .zip(module_name.chars())
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
    module_name: &str,
) -> Option<impl Iterator<Item = *const u8>> {
    let module = find_module_by_name(module_name)?;
    unsafe { Some(crate::find_pattern(pat, module.base, module.size)) }
}

/// Returns an iterator over all modules in the current process.
/// # Panics
/// If any module's name or path contain invalid UTF-16 sequence
#[cfg(feature = "alloc")]
pub fn modules() -> impl Iterator<Item = crate::types::ModuleAdvancedInfo> {
    use crate::types::{win::Teb, ModuleAdvancedInfo};

    Teb::current().peb.ldr.iter().map(|e| unsafe {
        ModuleAdvancedInfo {
            base: e.dll_base,
            size: e.image_size as usize,
            name: e.base_dll_name.to_string().unwrap(),
            path: e.full_dll_name.to_string().unwrap(),
        }
    })
}

extern "C" {
    fn AllocConsole();
    fn FreeConsole();
    fn FreeLibraryAndExitThread(lib: usize, code: u32) -> !;
}

#[link(name = "user32")]
extern "C" {
    fn GetAsyncKeyState(key: KeyCode) -> u16;
}

/// Allocates new console.
pub fn alloc_console() {
    unsafe {
        AllocConsole();
    }
}

/// Frees the console.
pub fn free_console() {
    unsafe {
        FreeConsole();
    }
}

/// Frees the library and exits current thread.
pub fn free_library_and_exit_thread(lib: usize, code: u32) -> ! {
    unsafe { FreeLibraryAndExitThread(lib, code) }
}

/// Returns the state of the key
/// Internally uses `GetAsyncKeyState`
pub fn key_state(key: KeyCode) -> u16 {
    unsafe { GetAsyncKeyState(key) }
}
