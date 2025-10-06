use std::{ffi::CString, os::raw::c_char};

unsafe extern "C" {
    fn hellothere(name: *const i8);
}

pub unsafe fn convert_str(input: &str) -> *mut c_char {
    return CString::new(input).unwrap().into_raw();
}

fn main() {
    println!("About to call into C++");
    unsafe {
        let s_ptr = convert_str("Shane");
        hellothere(s_ptr);
        drop(CString::from_raw(s_ptr));
    }
}