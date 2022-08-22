use memflex::{external::open_process_by_name, types::win::ProcessRights};

fn main() {
    let p = open_process_by_name(
        "Calculator.exe",
        false,
        ProcessRights::ALL_ACCESS
    ).unwrap();

    let pat = p.create_pattern_in_module(
        0x7FFE40CA1360, // 1360
        "tracelogging.dll",
        None
    ).unwrap().unwrap();

    println!("{:}", pat.to_ida_style());
}