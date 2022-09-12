#[no_mangle]
pub fn actual_add(a: i32, b: i32) -> i32 {
    (a + b) * 10
}

memflex::function! {
    // Offset could change if you compile the example
    fn ADDER(i32, i32) -> i32 = "function.exe"#0x30A0;

    fn MIXER(f32, f32, f32) -> u32 = "function.exe"%"48 81 EC B8 00 00 00 F3";
}

fn main() {
    let v1 = actual_add(10, 15);
    let v2 = ADDER(10, 15);
    assert_eq!(v1, v2);

    // MIXER.force(); // - Not required, it will early resolve the signature

    let v1 = mix_three(1., 2., 3.);
    let v2 = MIXER(1., 2., 3.);
    assert_eq!(v1, v2);
}

fn mix_three(a: f32, b: f32, c: f32) -> u32 {
    let n1 = (a * b) % c;
    let n2 = c / b + a;
    let mut out = n1.to_be_bytes();
    out.iter_mut()
        .zip(n2.to_be_bytes())
        .for_each(|(a, b)| *a ^= b);

    u32::from_be_bytes(out)
}
