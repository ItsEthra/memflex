use memflex::{external::open_process_by_id, types::ProcessAccess};

fn main() {
    let p = open_process_by_id(13484, false, ProcessAccess::PROCESS_ALL_ACCESS)
        .unwrap();

    for m in p.modules().unwrap() {
        dbg!(m);
    }
}
