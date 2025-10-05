#if _WIN32
#include <windows.h>
#endif
#include "llama.h"

#if _WIN32
    #ifdef __cplusplus
        #define FFI_EXPORT extern "C" __declspec(dllexport)
    #else
        #define FFI_EXPORT __declspec(dllexport)
    #endif
#else
    #ifdef __cplusplus
        #define FFI_EXPORT extern "C" 
    #else
        #define FFI_EXPORT
    #endif
#endif

FFI_EXPORT void start_llama(char* path_model, struct llama_model_params model_params);
FFI_EXPORT char* run_generation(char* promptc, int n_predict, struct llama_context_params context_params, struct llama_sampler_chain_params sampler_params);
FFI_EXPORT struct llama_model_params get_default_model_params();
FFI_EXPORT struct llama_context_params get_default_context_params();
FFI_EXPORT struct llama_sampler_chain_params get_default_sampler_params();
FFI_EXPORT void free_string(char* str);