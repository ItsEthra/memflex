use memflex::external::ProcessIterator;

fn main() {
    for p in ProcessIterator::new().unwrap() {
        if p.name == "firefox" {
            let p = p.open().unwrap();
            for m in p.modules().unwrap() {
                println!("{} {:p}", m.name, m.base);
            }
            return;
        }
    }
}
