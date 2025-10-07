#include "stdio.h"
#include <string>
#include <vector>
#include <stdlib.h>
#include <cstring>
#include <atomic>
#include <mutex>

#include "rsllama.h"


static struct llama_model* model = NULL;
static const struct llama_vocab* vocab = NULL;

std::atomic<bool> ok_to_generate;
std::mutex state_change_mutex;

FFI_EXPORT void start_llama(char* path_model, struct llama_model_params model_params) {
    // start model backend
    ggml_backend_load_all();
    // initialize the specific model
    model = llama_model_load_from_file(path_model, model_params);
    // initialize vocabulary
    vocab = llama_model_get_vocab(model);
    ok_to_generate.store(true);
}

FFI_EXPORT void stop_llama() {
    ok_to_generate.store(false);
    state_change_mutex.lock();
    llama_model_free(model);
}

FFI_EXPORT char* run_generation(char* promptc, int n_predict, struct llama_context_params context_params, struct llama_sampler_chain_params sampler_params) {
    std::string prompt(promptc);

    if (model == NULL || vocab == NULL) {
        fprintf(stderr, "Model or vocabulary not initialized. Call start_llama() first, or see why said call failed to initialize.\n");
        return NULL;
    }

    // tokenize the prompt, allocating enough space for the tokens
    const int n_prompt = -llama_tokenize(vocab, prompt.c_str(), prompt.size(), NULL, 0, true, true);
    std::vector<llama_token> prompt_tokens(n_prompt);
    if (llama_tokenize(vocab, prompt.c_str(), prompt.size(), prompt_tokens.data(), prompt_tokens.size(), true, true) < 0) {
        fprintf(stderr, "%s: error: failed to tokenize the prompt\n", __func__);
        return NULL;
    }    

    // inittialize the context, sizing according to prompt and allowed prediction token size
    context_params.n_ctx = n_prompt + n_predict - 1; // make context big enough to hold the prompt and desired # of prediction tokens
    context_params.n_batch = n_prompt; // n_batch is the maximum number of tokens that can be processed in a single call to llama_decode
    struct llama_context* ctx = llama_init_from_model(model, context_params);  

    // create a sampler for the generation
    struct llama_sampler* sampler = llama_sampler_chain_init(sampler_params);
    llama_sampler_chain_add(sampler, llama_sampler_init_greedy());

    // prepare a batch for the prompt
    llama_batch batch = llama_batch_get_one(prompt_tokens.data(), prompt_tokens.size());    
    
    // main decode loop
    std::string generation;
    const auto t_main_start = ggml_time_us();
    int n_decode = 0;
    llama_token new_token_id;
    state_change_mutex.lock();
    for (int n_pos = 0; n_pos + batch.n_tokens < n_prompt + n_predict && ok_to_generate.load(); ) {
        // evaluate the current batch with the transformer model
        if (llama_decode(ctx, batch)) {
            fprintf(stderr, "%s : failed to eval, return code %d\n", __func__, 1);
            return NULL;
        }

        n_pos += batch.n_tokens;

        // sample the next token
        {
            new_token_id = llama_sampler_sample(sampler, ctx, -1);

            // is it an end of generation?
            if (llama_vocab_is_eog(vocab, new_token_id)) {
                break;
            }

            char buf[128];
            int n = llama_token_to_piece(vocab, new_token_id, buf, sizeof(buf), 0, true);
            if (n < 0) {
                fprintf(stderr, "%s: error: failed to convert token to piece\n", __func__);
                return NULL;
            }
            std::string s(buf, n);
            generation += s;

            // prepare the next batch with the sampled token
            batch = llama_batch_get_one(&new_token_id, 1);

            n_decode += 1;
        }
    }
    state_change_mutex.unlock();
    printf("\n");
    const auto t_main_end = ggml_time_us();    
    
    // log stats
    fprintf(stderr, "%s: decoded %d tokens in %.2f s, speed: %.2f t/s\n",
            __func__, n_decode, (t_main_end - t_main_start) / 1000000.0f, n_decode / ((t_main_end - t_main_start) / 1000000.0f));    
    fprintf(stderr, "\n");
    llama_perf_sampler_print(sampler);
    llama_perf_context_print(ctx);
    fprintf(stderr, "\n");    

    // free resources
    llama_sampler_free(sampler);
    llama_free(ctx);

    char *result_copy = new char[generation.size() + 1];
    strcpy(result_copy, generation.c_str());
    return result_copy;
}