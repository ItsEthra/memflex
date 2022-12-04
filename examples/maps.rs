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
    use memflex::{external::open_process_by_id, types::win::PROCESS_ALL_ACCESS};

    let p = open_process_by_id(19076, false, PROCESS_ALL_ACCESS).unwrap();
    dbg!(p.maps());
}
