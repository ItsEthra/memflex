use std::mem::transmute_copy;

mod game {
    #[repr(C)]
    struct FooVmt {
        idx0: extern "C" fn(&Foo) -> i32,
        idx1: extern "C" fn(&mut Foo, i32) -> i32,
    }

    #[repr(C)]
    pub struct Foo {
        vmt: &'static FooVmt,
        health: i32,
    }

    impl Foo {
        pub fn new() -> Self {
            Foo {
                vmt: &FooVmt {
                    idx0: Foo::get_health,
                    idx1: Foo::set_health,
                },
                health: 100,
            }
        }
    }

    impl Foo {
        pub extern "C" fn get_health(&self) -> i32 {
            self.health
        }

        pub extern "C" fn set_health(&mut self, new: i32) -> i32 {
            let old = self.health;
            self.health = new;
            old
        }
    }
}

struct CFoo([u8; 0x10]);

memflex::interface! {
    pub trait IFoo impl for CFoo {
        extern fn get_health() -> i32 = #0;
        extern fn set_health(new: i32) -> i32 = #1;
    }
}

#[test]
fn test_interface() {
    let foo = game::Foo::new();

    unsafe {
        let this = transmute_copy::<_, CFoo>(&foo);
        assert_eq!(this.get_health(), 100);
        assert_eq!(this.set_health(50), 100);
        assert_eq!(this.get_health(), 50);
        assert_eq!(CFoo::FUNCTION_COUNT, 2);
    }
}

struct CBar(u32);
impl CBar {
    pub fn bar(&self, a: f32, b: f32) {
        let c = (a * b) + (a % b);
        println!("Hello: {}", self.0 as f32 + c);
    }
}

memflex::interface! {
    trait IBar impl for CBar {
        fn func(a: f32, b: f32) = %"48 81 EC A8 00 00 00 F3", "interface-942106fee1176959.exe";
    }
}

#[test]
fn test_interface_sig() {
    let bar = CBar(5);
    
    bar.bar(15., 30.);
    bar.func(15., 30.);

    let m = memflex::internal::current_module().unwrap();

    unsafe {
        let p = memflex::create_pattern(CBar::bar as usize as _, m.base, m.size, None)
            .unwrap();
        panic!("{:?}", p.to_ida_style());
    }
}
