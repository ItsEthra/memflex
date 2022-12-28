use memflex::resolve_multilevel;
use std::mem::zeroed;

#[repr(C)]
struct Mp1 {
    _pad: [u8; 0x10],
    val: Box<Mp2>,
}

#[repr(C)]
struct Mp2 {
    _pad: [u8; 0x50],
    val: Box<Mp3>,
}

#[repr(C)]
struct Mp3 {
    _pad: [u8; 8],
    val: Box<i32>,
}

#[test]
fn test_multilevel() {
    unsafe {
        let m1 = Mp1 {
            val: Box::new(Mp2 {
                _pad: zeroed(),
                val: Box::new(Mp3 {
                    _pad: zeroed(),
                    val: Box::new(1337),
                }),
            }),
            _pad: zeroed(),
        };

        let v = resolve_multilevel::<i32>(&m1 as *const _ as _, &[0x10, 0x50, 0x8]);
        assert_eq!(*v, 1337);
    }
}
