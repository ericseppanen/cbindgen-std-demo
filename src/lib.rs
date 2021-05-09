use std::ffi::CString;

#[repr(C)]
pub struct TestStruct {
    pub x: u8,
    pub y: u64,
}

pub const SOME_CONSTANT: u64 = 12345678;

#[repr(C)]
#[derive(PartialEq)]
pub enum TestEnum {
    One = 1,
    Two = 2,
    Three = 3,
}

#[no_mangle]
pub extern "C" fn fill_buffer(buf: *mut u8, len: usize) {
    assert!(!buf.is_null());
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, len as usize) };

    let s = format!("hello, {}", "world");

    let s = CString::new(s).unwrap();
    let sb = s.as_bytes_with_nul();

    buf[0..sb.len()].copy_from_slice(sb);
}

#[no_mangle]
pub extern "C" fn fill_struct(ptr: *mut TestStruct) {
    let test_struct = unsafe { ptr.as_mut().unwrap() };
    test_struct.x = 42;
    test_struct.y = 12345678;
}

#[no_mangle]
pub extern "C" fn handle_enum(x: TestEnum) -> bool {
    x == TestEnum::Two
}
