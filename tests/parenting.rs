use memflex::Child;

memflex::makestruct! {
    #[derive(Debug, Default, PartialEq)]
    pub struct Foo {
        a: f32,
        b: bool
    }
    
    #[derive(Debug, Default, PartialEq)]
    pub struct Bar : Foo {
        c: u64,
        d: i8
    }

    #[derive(Debug, Default, PartialEq)]
    pub struct Quz : Foo {
        e: f64,
        f: u16
    }
}

#[test]
fn test_parenting() {
    let mut orig = Bar::default();
    orig.a = 1.0;
    orig.b = true;
    orig.c = 1337;
    orig.d = -15;    

    unsafe {
        let parent = orig.upcast();
        let child = memflex::downcast::<Bar, _>(parent);
        assert_eq!(child, &orig);
    }
}