#[cfg(unix)]
fn main() {
    use memflex::{types::Protection, internal::{allocate, pid}};

    _ = dbg!(pid(), allocate(None, 0x2000, Protection::RWX));
    loop {}
}

#[cfg(windows)]
fn main() {
    use memflex::external::ProcessIterator;

    for p in ProcessIterator::new().unwrap() {
        println!("{}", p.path);
    }
}
