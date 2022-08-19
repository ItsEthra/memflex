memflex::makestruct! {
    #[derive(Default)]
    struct Parent {
        first: i32
    }

    #[derive(Default)]
    struct Child : Parent {
        second: i32
    }
}

#[test]
fn test_makestruct() {
    let child = Child::default();
    assert_eq!(child.first, child.parent.first);
}

memflex::interface! {
    trait Tux {
        fn f1() -> i32 = 0;
        fn f2() -> i32 = 1;
    }

    trait Qur {
        fn f3(a: i32) = 0;
        fn f4(a: i32) = 1;
    }
}

memflex::makestruct! {
    #[derive(Default)]
    struct Foo impl Tux {
        t1: i32,
        t2: f32   
    }

    #[derive(Default)]
    struct Bar impl Qur(dyn Foo) : pub Foo {
        t3: u64,
        t4: f64
    }
}

#[test]
fn test_makestruct_with_interface() {
    assert_eq!(Foo::INDEX_OFFSET, 0);
    assert_eq!(Bar::INDEX_OFFSET, 2);
}