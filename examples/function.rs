#[no_mangle]
pub fn actual_add(a: i32, b: i32) -> i32 {
    (a + b) * 10
}

memflex::function! {
    fn ADDER(i32, i32) -> i32 = "function.exe"#0x13C0;
}

fn main() {
    let v1 = actual_add(10, 15);
    let v2 = ADDER(10, 15);
    assert_eq!(v1, v2);
}
