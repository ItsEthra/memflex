use memflex::external::OwnedProcess;

fn main() {
    
    let p = OwnedProcess::new(45632);

    let mut buf = [0u8; 4];
    dbg!(p.read_buf(0x7ffe6ed79614, &mut buf));


    dbg!(i32::from_ne_bytes(buf));
}
