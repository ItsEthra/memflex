use memflex::{Flow, Ptr, PtrStatic};

fn main() {
    unsafe {
        let val = 5;
        let s = StructWithPtr {
            icanbenull: Ptr::new_unchecked(&val),
        };
        sub(s);
    }
}

struct StructWithPtr {
    icanbenull: PtrStatic<u32>,
}

fn sub(s: StructWithPtr) -> Flow<()> {
    let value = s.icanbenull?;
    println!(
        "If you see this, then ptr is not null and the value is {}",
        *value
    );

    let out = inner(Ptr::from(Box::new(true)))?;

    println!("After inner call: {out}");

    Flow::Done(())
}

fn inner(other: PtrStatic<bool>) -> Flow<bool> {
    let value = *other?;
    println!("If you see this, then other is: {value}");

    Flow::Done(!value)
}