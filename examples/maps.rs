#[cfg(unix)]
fn main() {
    use memflex::external::ProcessIterator;

    for d in ProcessIterator::new().unwrap() {
        dbg!(d.name);
    }
}
