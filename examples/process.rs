use memflex::{external::{open_process_by_id, ThreadIterator}, types::ProcessAccess};

fn main() {
    let p = open_process_by_id(2396, false, ProcessAccess::PROCESS_ALL_ACCESS).unwrap();

    for t in ThreadIterator::new(p.id()).unwrap() {
        dbg!(t);
    }
}
