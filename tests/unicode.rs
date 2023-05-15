use memflex::{types::win::UnicodeString, unicode_string};

#[test]
fn test_unicode_macros() {
    const TEST_STRING: &'static str = "Memflex Unicode String";
    const UNICODE_STRING: UnicodeString = unicode_string!(TEST_STRING);

    assert_eq!(UNICODE_STRING.len(), TEST_STRING.len());
    assert_eq!(UNICODE_STRING.bytes_len(), TEST_STRING.len() * 2);

    assert_eq!(unsafe { UNICODE_STRING.to_string() }.unwrap(), TEST_STRING);
}
