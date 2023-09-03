#[no_mangle]
static mut SOME_INT: i32 = 15;

memflex::global! {
    // Offset could change if you compile the example
    extern GLOBAL_INT: i32 = "global_int.exe"#0x2B000;
}

fn main() {
    unsafe {
        assert_eq!(SOME_INT, *GLOBAL_INT);
        SOME_INT += 10;
        assert_eq!(SOME_INT, *GLOBAL_INT);
    }
}
