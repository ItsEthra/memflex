#[cfg(unix)]
fn main() {
    use memflex::{
        internal::{allocate, pid},
        types::Protection,
    };

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
