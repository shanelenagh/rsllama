use std::{io, io::BufRead, ffi::CString, os::raw::c_char};
use argh::FromArgs;
// Suppress warnings from the crappy auto-generated llama.cpp binding code from bindgen
#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused_imports,
    unnecessary_transmutes, unsafe_op_in_unsafe_fn, dead_code)]
mod llama {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // llama.cpp bindgen bindings generated from build.rs
}
use llama::{
    llama_model_default_params, llama_context_default_params, llama_sampler_chain_default_params, 
    llama_context_params, llama_sampler_chain_params, start_llama, stop_llama, run_generation
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

unsafe fn convert_str(input: &str) -> *mut c_char {
    return CString::new(input).unwrap().into_raw();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: LlamaArgs = argh::from_env();
    unsafe {
        let mut model_params = llama_model_default_params();
        model_params.n_gpu_layers = 99;
        start_llama(convert_str(&args.model), model_params);
        let model_context: llama_context_params = llama_context_default_params();
        let sampler_params: llama_sampler_chain_params = llama_sampler_chain_default_params();        
        if let Some(ref prompt) = args.prompt { // Single prompt given
            let _ = run_llama(&prompt, &args, &model_context, &sampler_params);
        } else { // No prompt given ==> Read lines from stdin
            let stdin_handle = io::stdin().lock();
            println!(">>>> Enter prompt/question:");
            for line_result in stdin_handle.lines() {
                let line = line_result?;
                if line.trim().is_empty() {
                    break;
                }
                let _ = run_llama(&line, &args, &model_context, &sampler_params);
                println!(">>>> Enter next prompt/question:");
            }
        }
        stop_llama();
    }
    Ok(())
}

fn run_llama(prompt: &str, args: &LlamaArgs, model_context: &llama_context_params, sampler_params: &llama_sampler_chain_params) -> Result<(), Box<dyn std::error::Error>> {
    println!(">>>>>>>>>> about to run gen for model [{}] and prompt [{}]", args.model, prompt);
    unsafe {
        let result = run_generation(convert_str(prompt), args.tokens, *model_context, *sampler_params);
        println!("Got result: {}", CString::from_raw(result).into_string()?);
    }
    Ok(())
}