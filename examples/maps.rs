#[cfg(unix)]
fn main() {
    use memflex::external::{ProcessIterator, find_process_by_id, find_process_by_name};

    for p in find_process_by_name("alacritty").unwrap().maps() {
        dbg!(p);
    }

}
