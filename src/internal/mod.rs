#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

use crate::Matcher;

/// Returns an information about current module
/// # Behavior
/// Looks up module by looking up RIP register.
/// Can return `None` if the module was manually mapped and not linked in ldr.
#[cfg(feature = "alloc")]
pub fn current_module() -> Option<crate::types::ModuleInfoWithName> {
    let mut rip: usize;
    unsafe {
        core::arch::asm!("lea {}, [rip]", out(reg) rip);

        modules().find(|m| rip < m.base as usize + m.size && rip > m.base as usize)
    }
}

/// Searches for a pattern in the specified module.
pub fn find_pattern_in_module(
    pat: impl Matcher,
    module_name: &str,
) -> Option<impl Iterator<Item = *const u8>> {
    let module = find_module_by_name(module_name)?;
    unsafe { Some(crate::find_pattern(pat, module.base, module.size)) }
}
