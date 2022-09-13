#[cfg(unix)]
fn main() {
    use memflex::external::ProcessIterator;

    for p in ProcessIterator::new().unwrap() {
        dbg!(p.name);
    }
}
