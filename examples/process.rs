use memflex::external::OwnedProcess;

fn main() {
    let p = OwnedProcess::open_by_id(13484, false, OwnedProcess::PROCESS_ALL_ACCESS)
        .unwrap();

    dbg!(p.read::<u8>(0x0007FF7F016D524).unwrap());
}