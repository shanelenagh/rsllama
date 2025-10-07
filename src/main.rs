#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(unsafe_op_in_unsafe_fn)]
use std::{ffi::CString, os::raw::c_char};
use argh::FromArgs;

include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // bindgen llama.cpp bindings

#[derive(FromArgs)]
/// Llama.cpp wrapper
struct LlamaArgs {
    /// model file (GGUF format)
    #[argh(option, short='m')]
    model: String,
    /// prompt or question to ask model (leaving out causes it to read from stdin)
    #[argh(option, short='p')]
    prompt: Option<String>
}

pub unsafe fn convert_str(input: &str) -> *mut c_char {
    return CString::new(input).unwrap().into_raw();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: LlamaArgs = argh::from_env();
    unsafe {
        let mut model_params: llama_model_params = llama_model_default_params();
        model_params.n_gpu_layers = 99;
        start_llama(convert_str(&args.model), model_params);
        if let Some(prompt) = args.prompt {
            let model_context: llama_context_params = llama_context_default_params();
            let sampler_params: llama_sampler_chain_params = llama_sampler_chain_default_params();
            println!(">>>>>>>>>> about to run gen for model [{}] and prompt [{}]", args.model, prompt);
            let result = run_generation(convert_str(&prompt), 1000, model_context, sampler_params);
            println!("Got result: {}", CString::from_raw(result).into_string()?);
        }
    }
    Ok(())
}