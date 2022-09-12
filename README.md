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

#[cfg(windows)]
{
    let first = memflex::internal::find_pattern_in_module(ida, "ntdll.dll").unwrap().next();
    let last = memflex::internal::find_pattern_in_module(peid, "ntdll.dll").unwrap().last();
}
```
* Module searching
```rust
let module = memflex::internal::find_module_by_name("ntdll.dll");
// module.size, module.base
```
* Macros for emulating C++ behavior
```rust
#[repr(C)]
pub struct ConcreteType {
    vmt: usize
};

memflex::interface! {
    pub trait IPlayer impl for ConcreteType {
        // Notice missing `&self`, this is intentional and macro will implicitly add it.
        // Functions without `&self` in interface doesn't make much sense.
        extern "C" fn get_health() -> i32 = #0; // 0 - Index of the virtual function.

        // *Returns old health*
        extern "C" fn set_health(new: i32) -> i32 = #1; // 1 - Index of the virtual function.
    }

    trait Foo {
        extern fn foo() = #0;
    }

    trait ParentVmt {
        fn f1() -> i32 = #0;
        fn f2() -> i32 = #1;
    }

    trait ChildVmt {
        fn f3(a: i32) = #0;
        fn f4(a: i32) = #1;
    }
}

memflex::makestruct! {
    // Attributes works as expected
    #[derive(Default)]
    struct Parent {
        // on fields as well
        // #[serde(skip)]
        first: f32
    }
    
    // `pub` means that `parent` field will be `pub`
    // but Deref<Target = Parent> implementation will be generated regardless.
    struct Child : pub Parent {
        second: i32
    }

    // Implements `Foo` interface on `Nested`
    struct Nested impl Foo : Child {
        third: bool
    }

    struct ParentWithVmt impl ParentVmt {
        vmt: usize,
        t1: f32,
        t2: bool
    }

    // By using `dyn ParentWithVmt`, child offsets all of their vfunc indices by the number of functions in `ParentWithVmt`,
    // should work with nested inheritance but hasn't been tested!
    struct ChildInheritsParentVmt impl ChildVmt(dyn ParentWithVmt) : pub ParentWithVmt {
        t3: u64,
        t4: i8
    }
}

memflex::global! {
    // Uses default ldr resolver
    pub static MY_GLOBAL: i32 = "ntdll.dll"#0x1000;
}

memflex::function! {
    // Function with offset from the module
    fn ADDER(i32, i32) -> i32 = "function.exe"#0x2790;

    // Function with signature
    fn MIXER(f32, f32, f32) -> u32 = "function.exe"%"48 81 EC B8 00 00 00 F3";
}


memflex::bitstruct! {
    struct SomeStruct : u8 {
        // Bits: 0, 1, 2
        first: 0..=2,
        // Bits: 3, 4, 5, 6, 7
        next: 3..=7,
    }
}

use memflex::types::TStr;
let zero_terminated: TStr = memflex::tstr!("Hello, World!");
```
