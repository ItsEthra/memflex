use memflex::external::ProcessIterator;

fn main() {
    for p in ProcessIterator::new().unwrap() {
        dbg!(p);
    }
}
