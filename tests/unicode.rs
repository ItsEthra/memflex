use memflex::{types::win::UnicodeString, unicode_string};

#[test]
fn test_unicode_macros() {
    const UNICODE_STRING: UnicodeString = unicode_string!("Memflex Unicode String");

    assert_eq!(UNICODE_STRING.len(), "Memflex Unicode String".len());
    assert_eq!(
        UNICODE_STRING.bytes_len(),
        "Memflex Unicode String".len() * 2
    );

    assert_eq!(
        unsafe { UNICODE_STRING.to_string() }.unwrap(),
        "Memflex Unicode String"
    );
}
