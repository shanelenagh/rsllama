use std::{io, io::BufRead, ffi::CString, os::raw::c_char};
use argh::FromArgs;
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused_imports)]
#[allow(unnecessary_transmutes)]
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(dead_code)]
mod llama {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // bindgen llama.cpp bindings
}
use llama::{
    llama_model_default_params, llama_context_default_params, llama_sampler_chain_default_params, 
    start_llama, stop_llama, run_generation
};


#[derive(FromArgs)]
/// Llama.cpp wrapper
struct LlamaArgs {
    /// model file (GGUF format)
    #[argh(option, short='m')]
    model: String,
    /// prompt or question to ask model (leaving out causes it to read from stdin)
    #[argh(option, short='p')]
    prompt: Option<String>,
    /// token count
    #[argh(option, short='t', default="1000")]
    tokens: i32
}

pub unsafe fn convert_str(input: &str) -> *mut c_char {
    return CString::new(input).unwrap().into_raw();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: LlamaArgs = argh::from_env();
    unsafe {
        let mut model_params = llama_model_default_params();
        model_params.n_gpu_layers = 99;
        start_llama(convert_str(&args.model), model_params);
        let model_context = llama_context_default_params();
        let sampler_params = llama_sampler_chain_default_params();        
        if let Some(prompt) = args.prompt { // Single prompt given
            println!(">>>>>>>>>> about to run gen for model [{}] and prompt [{}]", args.model, prompt);
            let result = run_generation(convert_str(&prompt), args.tokens, model_context, sampler_params);
            println!("Got result: {}", CString::from_raw(result).into_string()?);
        } else { // No prompt given ==> Read lines from stdin
            let stdin_handle = io::stdin().lock();
            println!(">>>> Enter prompt/question:");
            for line_result in stdin_handle.lines() {
                let line = line_result?;
                if line.trim().is_empty() {
                    break;
                }
                println!(">>>>>>>>>> about to run gen for model [{}] and prompt [{}]", args.model, line);
                let result = run_generation(convert_str(&line), args.tokens, model_context, sampler_params);
                println!("Got result: {}", CString::from_raw(result).into_string()?);
                println!(">>>> Enter next prompt/question:");
            }
        }
        stop_llama();
    }
    Ok(())
}