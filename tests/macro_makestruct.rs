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
