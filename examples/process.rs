use memflex::{
    external::{open_process_by_id, ThreadIterator},
    types::{ProcessRights, ThreadRights},
};

fn main() {
    let p = open_process_by_id(40424, false, ProcessRights::ALL_ACCESS).unwrap();

    for t in ThreadIterator::new(p.id()).unwrap() {
        let o = t.open(false, ThreadRights::ALL_ACCESS).unwrap();
        println!("{:X}", o.start_address().unwrap());
    }
}
