extern crate cmake;

fn main() {
    let target = cmake::Config::new("native/llama_cpp")
        .define("LLAMA_CURL", "OFF")
        .build();

    // Prints instructions for Cargo to link the native library
    println!("cargo:rustc-link-search=native={}/lib", target.display());
    println!("cargo:rustc-link-lib=dylib=llama"); 
}