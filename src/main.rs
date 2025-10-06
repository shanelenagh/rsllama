#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::{ffi::CString, os::raw::c_char};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// unsafe extern "C" {
//     fn hellothere(name: *const i8);
// }

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