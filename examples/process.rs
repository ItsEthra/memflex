use memflex::{external::open_process_by_id, types::win::ProcessRights, ida_pat};

fn main() {
    let p = open_process_by_id(
        44612,
        false,
        ProcessRights::ALL_ACCESS
    ).unwrap();

    let a = p.find_pattern_in_module(
        ida_pat!("40 53 48 83 EC 20 8B"),
        "tracelogging.dll"
    )
        .unwrap()
        .next()
        .unwrap();

    println!("{:X}", a);
}