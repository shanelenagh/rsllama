extern crate cmake;

fn main() {
    let target = cmake::Config::new("native")
        .define("LLAMA_CURL", "OFF")
        .build();

    // Prints instructions for Cargo to link the native library
    println!("cargo:rustc-link-search=native={}/lib", target.display());
    println!("cargo:rustc-link-search=native={}/build/Debug", target.display());
    println!("cargo:rustc-link-lib=static=rsllama"); 


    /*
     * Bindgen mapping
     */
    use std::env;
    use std::path::PathBuf;     
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("native/rsllama.h")
        .clang_arg("-Inative/llama_cpp/include")
        .clang_arg("-Inative/llama_cpp/ggml/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}