# Memflex - Memory hacking library

# Features
* Pattern matching
```rust
use memflex::{ida_pat, peid_pat};
// Pattern creation and parsing happens at compile time.
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
* Read/Write external memory
```rust
#[cfg(windows)]
if let Ok(p) = memflex::external::open_process_by_id(666, false, memflex::types::win::PROCESS_ALL_ACCESS) {
    let value = p.read::<u32>(0x7FFF)?;
    p.write(0x7FFF, 100_u32)?;
}

Ok::<_, memflex::MfError>(())
```
* Macros for emulating C++ behavior
```rust
#[repr(C)]
pub struct ConcreteType {
    vmt: memflex::VmtPtr
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

// Automatically wraps all structures in `#[repr(C)]`
// Virtual functions with inheritence is not tested(probably doesnt work).
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
    // This macro assumes msvc virtual parenting behavior when for each child a separate vmt is generated.
    struct ChildInheritsParentVmt impl ChildVmt(dyn ParentWithVmt) : pub ParentWithVmt {
        t3: u64,
        t4: i8
    }
}

memflex::global! {
    // Uses default ldr resolver on windows
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

// Null terminated strings
use memflex::types::TStr;
let zero_terminated: TStr = memflex::tstr!("Hello, World!");
```
