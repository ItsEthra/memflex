# Memflex - Memory hacking library

# Features
* Checked pointers
```rust
# use memflex::{Flow, Ptr};
pub struct StructWithPtr<'a> {
    pub thiscanbenull: Ptr<'a, u32>
}

pub fn sub(s: StructWithPtr) -> Flow<()> {
    let value = s.thiscanbenull?;
    
    println!(
        "If you see this, then ptr is not null and the value is {}",
        *value
    );

    // Here it checks if function actually returned a value
    let out = inner(Ptr::null())?;

    println!("After inner call: {out}");

    Flow::Done(())
}

fn inner(other: Ptr<bool>) -> Flow<bool> {
    // Here it checks if the pointer is zero and skips if it is
    let value = *other?;
    println!("If you see this, then other is: {value}");

    Flow::Done(!value)
}
```
* Pattern matching
```rust
# use memflex::{ida_pat, peid_pat};
let ida = ida_pat!("13 ? D1");
let peid = peid_pat!("13 ?? D1");

let first = memflex::internal::find_pattern_in_module(ida, "ntdll.dll").unwrap().next();
let last = memflex::internal::find_pattern_in_module(peid, "ntdll.dll").unwrap().last();
```
* Module searching
```rust
let module = memflex::internal::find_module_by_name("ntdll.dll");
// module.size, module.base
```