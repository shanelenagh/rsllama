#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(unsafe_op_in_unsafe_fn)]
use std::{ffi::CString, os::raw::c_char};

include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // llama.cpp bindings

pub unsafe fn convert_str(input: &str) -> *mut c_char {
    return CString::new(input).unwrap().into_raw();
}

fn main() {
    unsafe {
        let model_params: llama_model_params = llama_model_default_params();
        start_llama(convert_str("granite-3.3-2b-instruct-Q5_1.gguf"), model_params);
    }
}