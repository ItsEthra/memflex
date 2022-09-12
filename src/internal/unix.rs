use crate::types::ModuleBasicInfo;
use std::fs;

/// Searches for a module by its name.
/// # Behavior
/// Iterates `/proc/self/maps` finiding maps that ends with `name`.
/// Case sensetive.
pub fn find_module_by_name(name: &str) -> Option<ModuleBasicInfo> {
    let s = fs::read_to_string("/proc/self/maps").unwrap();

    let mut m_from = usize::MAX;
    let mut m_to = 0;

    for l in s.lines() {
        let map = l.split(' ').filter(|v| !v.is_empty()).collect::<Vec<_>>();
        if map.len() != 6 {
            continue;
        }

        let lib = map[5];
        if lib.ends_with(name) {
            let (from, to) = map[0].split_once('-').unwrap();
            let from = usize::from_str_radix(from, 16).unwrap();
            let to = usize::from_str_radix(to, 16).unwrap();

            m_from = from.min(m_from);
            m_to = to.max(m_to);
        }
    }

    if m_to == 0 {
        None
    } else {
        Some(ModuleBasicInfo {
            base: m_from as *const u8,
            size: m_to - m_from,
        })
    }
    
}

#[cfg(feature = "alloc")]
/// Returns an iterator over all modules in the process.
pub fn modules() -> impl Iterator<Item = crate::types::ModuleAdvancedInfo> {
    use crate::types::ModuleAdvancedInfo;
    use std::{collections::HashMap, path::PathBuf};

    let s = fs::read_to_string("/proc/self/maps").unwrap();
    let mut maps: HashMap<String, (usize, usize)> = HashMap::new();

    for l in s.lines() {
        let map = l.split(' ').filter(|v| !v.is_empty()).collect::<Vec<_>>();
        if map.len() != 6 {
            continue;
        }

        let lib = map[5];

        let (from, to) = map[0].split_once('-').unwrap();
        let from = usize::from_str_radix(from, 16).unwrap();
        let to = usize::from_str_radix(to, 16).unwrap();

        if fs::metadata(lib).is_ok() {
            let ent = maps.entry(lib.to_owned()).or_insert_with(|| (from, to));

            if from < ent.0 {
                ent.0 = from;
            } else if to > ent.1 {
                ent.1 = to;
            }
        }
    }

    maps.into_iter().filter_map(|(k, (from, to))| {
        let path = PathBuf::from(k);
        Some(ModuleAdvancedInfo {
            name: path.file_name()?.to_string_lossy().into_owned(),
            path: path.to_string_lossy().into_owned(),
            base: from as *const u8,
            size: to - from,
        })
    })
}
