use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        Console::{AllocConsole, FreeConsole},
        LibraryLoader::FreeLibraryAndExitThread,
    },
};

/// Searches for a module by its name.
/// # Behavior
/// Function iteraters over ldr searches for module entry (ascii case insensetive).
pub fn find_module_by_name(module_name: &str) -> Option<crate::types::ModuleInfo> {
    use crate::types::{win::Teb, ModuleInfo};

    unsafe {
        let mut iter = Teb::get().peb.as_ref().ldr.as_ref().iter();
        iter.next();

        iter.find_map(|e| {
            if e.base_dll_name
                .utf16()
                .zip(module_name.chars())
                .all(|(a, b)| a.eq_ignore_ascii_case(&b))
            {
                Some(ModuleInfo {
                    size: e.image_size as usize,
                    base: e.dll_base,
                })
            } else {
                None
            }
        })
    }
}

/// Returns an iterator over all modules in the current process.
/// # Panics
/// If any module's name or path contain invalid UTF-16 sequence
#[cfg(feature = "alloc")]
pub fn modules() -> impl Iterator<Item = crate::types::ModuleInfoWithName> {
    use crate::types::{win::Teb, ModuleInfoWithName};

    unsafe {
        Teb::get()
            .peb
            .as_ref()
            .ldr
            .as_ref()
            .iter()
            .map(|e| ModuleInfoWithName {
                base: e.dll_base,
                size: e.image_size as usize,
                name: e.base_dll_name.to_string().unwrap(),
            })
    }
}

/// Allocates new console.
pub fn alloc_console() -> bool {
    unsafe { AllocConsole().as_bool() }
}

/// Frees the console.
pub fn free_console() -> bool {
    unsafe { FreeConsole().as_bool() }
}

/// Frees the library and exits current thread.
pub fn free_library_and_exit_thread(lib: usize, code: u32) -> ! {
    unsafe { FreeLibraryAndExitThread(HINSTANCE(lib as _), code) }
}
