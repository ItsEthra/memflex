use std::mem::transmute_copy;

mod game {
    #[repr(C)]
    struct FooVmt {
        idx0: extern "C" fn(&Foo) -> i32,
        idx1: extern "C" fn(&mut Foo, i32) -> i32,
        idx2: extern "C" fn(&Foo) -> &i32,
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
                    idx2: Foo::get_health_ref,
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

        pub extern "C" fn get_health_ref(&self) -> &i32 {
            &self.health
        }
    }
}

struct CFoo([u8; 0x10]);

memflex::interface! {
    pub trait IFoo impl for CFoo {
        extern fn get_health() -> i32 = #0;
        extern fn set_health(new: i32) -> i32 = #1;
        extern fn get_health_ref() -> &'this i32 = #2;
    }
}

#[test]
#[allow(clippy::disallowed_names)]
fn test_interface() {
    let foo = game::Foo::new();

    unsafe {
        let this = transmute_copy::<_, CFoo>(&foo);
        assert_eq!(this.get_health(), 100);
        assert_eq!(this.set_health(50), 100);
        assert_eq!(this.get_health(), 50);
        assert_eq!(CFoo::FUNCTION_COUNT, 3);
    }
}
