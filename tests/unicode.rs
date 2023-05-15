use memflex::{types::win::UnicodeString, unicode_string};

#[test]
fn test_unicode_macros() {
    const TEST_STRING: &'static str = "Memflex Unicode String";
    const UNICODE_STRING: UnicodeString = unicode_string!(TEST_STRING);

    assert_eq!(UNICODE_STRING.len(), TEST_STRING.len());
    assert_eq!(UNICODE_STRING.bytes_len(), TEST_STRING.len() * 2);

    assert_eq!(unsafe { UNICODE_STRING.to_string() }.unwrap(), TEST_STRING);
}

#[test]
fn test_unicode_equality() {
    const FIRST_STRING: UnicodeString = unicode_string!("Memflex Unicode String");
    const SECOND_STRING: UnicodeString = unicode_string!("Memflex");
    const DIFFERENT_STRING: UnicodeString = unicode_string!("Memflex Unicode String 2");

    assert_eq!(FIRST_STRING, FIRST_STRING);
    assert_eq!(DIFFERENT_STRING, DIFFERENT_STRING);
    assert_eq!(SECOND_STRING, SECOND_STRING);

    assert_ne!(FIRST_STRING, DIFFERENT_STRING);
    assert_ne!(SECOND_STRING, DIFFERENT_STRING);
    assert_ne!(FIRST_STRING, SECOND_STRING);

    let rust_string = "Memflex Unicode String".to_string();
    let utf16_string = rust_string.encode_utf16().collect::<Vec<_>>();
    let runtime_unicode_string = UnicodeString::new(
        (rust_string.len() * 2) as _,
        (rust_string.len() * 2) as _,
        utf16_string.as_ptr(),
    );

    assert_eq!(runtime_unicode_string, FIRST_STRING);
    assert_ne!(runtime_unicode_string, DIFFERENT_STRING);
    assert_ne!(runtime_unicode_string, SECOND_STRING);
}
