#[cfg(windows)]
fn main() {
    use memflex::types::win::Teb;

    unsafe {
        let ldr = Teb::get().peb.as_ref().ldr.as_ref();
        for e in ldr.iter() {
            println!("{}", e.base_dll_name.to_string().unwrap());
        }
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This example can only run on windows");
}
