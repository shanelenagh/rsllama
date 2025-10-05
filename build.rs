extern crate cmake;

fn main() {
    let target = cmake::Config::new("native")
        .define("LLAMA_CURL", "OFF")
        .build();

    // Prints instructions for Cargo to link the native library
    println!("cargo:rustc-link-search=native={}/lib", target.display());
    println!("cargo:rustc-link-search=native={}/build/Debug", target.display());
    println!("cargo:rustc-link-lib=static=rsllama"); 
}