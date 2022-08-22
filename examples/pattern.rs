use memflex::internal::create_pattern_in_module;

fn main() {
    let p = create_pattern_in_module(mix_three as usize as _, "pattern.exe", None);
    dbg!(p.is_some());
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