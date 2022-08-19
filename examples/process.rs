use memflex::external::OwnedProcess;

fn main() {
    let p = OwnedProcess::open_by_id(13484, false, OwnedProcess::PROCESS_ALL_ACCESS)
        .unwrap();

    dbg!(p.write::<u64>(0x00000B417CFFAB0, 0xFFFFFFFF).unwrap());
}