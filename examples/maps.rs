#[cfg(unix)]
fn main() {
    use memflex::internal::find_module_by_name;

    _ = dbg!(find_module_by_name("asdmaps"));

    println!("{}", unsafe { libc::getpid() });

    loop {}
}
