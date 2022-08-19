static mut SOME_INT: i32 = 15;

memflex::global! {
    // Offset could change if you compile the example
    static GLOBAL_INT: i32 = "global_int.exe"#0x29000;
}

fn main() {
    unsafe {
        assert_eq!(SOME_INT, *GLOBAL_INT);
    }
}
