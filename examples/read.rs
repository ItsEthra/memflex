use memflex::external::find_process_by_name;

fn main() {
    let p = find_process_by_name("dummy").unwrap();
    dbg!(p.read::<u8>(0x564ce6d10eb0).unwrap());
}
