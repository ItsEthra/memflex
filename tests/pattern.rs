use memflex::{code_pat, ida_pat, peid_pat, Matcher, Pattern};

#[test]
fn test_pattern_search() {
    let memory: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    const IDA: Pattern<4> = ida_pat!("02 03 ? 05");
    let peid = peid_pat!("02 03 ?? 05");
    let code = code_pat!(b"\x02\x03\x00\x05", "xx?x");

    assert_eq!(IDA.len(), 4);
    assert_eq!(peid.len(), 4);
    assert_eq!(code.len(), 4);

    assert!(memory.windows(IDA.len()).any(|t| IDA.matches(t)));
    assert!(memory.windows(peid.len()).any(|t| peid.matches(t)));
    assert!(memory.windows(code.len()).any(|t| code.matches(t)));
}
