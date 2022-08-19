use memflex::{Flow, Ptr};

fn main() {
    let val = 5;
    let s = StructWithPtr(Ptr::new(&val));
    sub(s);
}

struct StructWithPtr<'a>(Ptr<'a, u32>);

fn sub(s: StructWithPtr) -> Flow<()> {
    let value = s.0?;
    println!(
        "If you see this, then ptr is not null and the value is {}",
        *value
    );

    let out = inner(Ptr::null())?;

    println!("After inner call: {out}");

    Flow::Done(())
}

fn inner(other: Ptr<bool>) -> Flow<bool> {
    let value = *other?;
    println!("If you see this, then other is: {value}");

    Flow::Done(!value)
}