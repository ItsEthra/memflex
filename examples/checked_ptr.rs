#[cfg(feature = "nightly")]
use memflex::Ptr;

#[cfg(feature = "nightly")]
fn main() {
    let val = 5;
    let s = nightly::StructWithPtr {
        thiscanbenull: Ptr::new(&val),
    };
    nightly::sub(s);
}

#[cfg(not(feature = "nightly"))]
fn main() {
    panic!("This example only run with nightly feature and compile");
}

#[cfg(feature = "nightly")]
mod nightly {
    use memflex::{Flow, Ptr};

    pub struct StructWithPtr<'a> {
        pub thiscanbenull: Ptr<'a, u32>,
    }

    pub fn sub(s: StructWithPtr) -> Flow<()> {
        let value = s.thiscanbenull?;

        println!(
            "If you see this, then ptr is not null and the value is {}",
            *value
        );

        let out = inner(Ptr::from(Box::new(true)))?;
        println!("After inner call: {out}");

        Flow::Null
    }

    fn inner(other: Ptr<bool>) -> Flow<bool> {
        let value = *other?;
        println!("If you see this, then other is: {value}");

        Flow::Done(!value)
    }
}
