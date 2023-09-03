#[cfg(windows)]
fn main() {
    use memflex::{
        iter_list,
        types::win::{LdrDataTableEntry, Teb},
    };

    unsafe {
        let ldr = Teb::get().peb.ldr;
        for e in iter_list!(
            &ldr.in_memory_order_list,
            LdrDataTableEntry,
            in_memory_order_links
        ) {
            println!("{}", e.base_dll_name.to_string().unwrap());
        }
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This example can only run on windows");
}
