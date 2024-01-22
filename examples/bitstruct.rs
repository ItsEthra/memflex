fn main() {
    //        76543210
    let a = 0b11010111;

    let from = 2;
    let to = 5;

    let mask = !0u32 << (to - from + 1);
    let v = mask.rotate_left(from);
    let b = (a & v) | ((0b1010 & !mask) << from);
    println!("{b:b}");
}
