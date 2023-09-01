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
    pub struct Quz : Bar {
        e: f64,
        f: u16
    }
}

#[test]
fn test_parenting() {
    let mut orig = Quz::default();
    orig.a = 1.0;
    orig.b = true;
    orig.c = 1337;
    orig.d = -15;
    orig.e = 3.1;
    orig.f = 9;

    assert_eq!(orig.a, orig.parent.a);
}
