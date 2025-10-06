//use cty;
use std::ffi::CString;
use std::os::raw::c_char;

// #[repr(C)]
// pub struct llama_model_params {
//     // // NULL-terminated list of devices to use for offloading (if NULL, all available devices are used)
//     // ggml_backend_dev_t * devices;

//     // // NULL-terminated list of buffer types to use for tensors that match a pattern
//     // const struct llama_model_tensor_buft_override * tensor_buft_overrides;

//     pub n_gpu_layers: cty::int32_t, // number of layers to store in VRAM
//     // enum llama_split_mode split_mode; // how to split the model across multiple GPUs

//     // the GPU that is used for the entire model when split_mode is LLAMA_SPLIT_MODE_NONE
//     pub main_gpu: cty::int32_t,

//     // // proportion of the model (layers or rows) to offload to each GPU, size: llama_max_devices()
//     // const float * tensor_split;

//     // // Called with a progress value between 0.0 and 1.0. Pass NULL to disable.
//     // // If the provided progress_callback returns true, model loading continues.
//     // // If it returns false, model loading is immediately aborted.
//     // llama_progress_callback progress_callback;

//     // // context pointer passed to the progress callback
//     // void * progress_callback_user_data;

//     // // override key-value pairs of the model meta data
//     // const struct llama_model_kv_override * kv_overrides;

//     // Keep the booleans together to avoid misalignment during copy-by-value.
//     pub vocab_only: bool,      // only load the vocabulary, no weights
//     pub use_mmap: bool,        // use mmap if possible
//     pub use_mlock: bool,       // force system to keep model in RAM
//     pub check_tensors: bool,   // validate model tensor data
//     pub use_extra_bufts: bool // use extra buffer types (used for weight repacking)
// }

unsafe extern "C" {
    //fn start_llama(path_model: *const i8, model_params: llama_model_params);
    fn hellothere(name: *const i8);
}

pub unsafe fn convert_str(input: &str) -> *mut c_char {
    //let c_str = CString::new(input).unwrap().into_raw();
    return CString::new(input).unwrap().into_raw(); //c_str;
}

fn main() {
    unsafe {
        let s_ptr = convert_str("Shane");
        hellothere(s_ptr);
        drop(CString::from_raw(s_ptr));
    }
}