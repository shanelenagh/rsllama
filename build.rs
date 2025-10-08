extern crate cmake;
use std::{fs, path::Path};
use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = cmake::Config::new("native")
        .define("LLAMA_CURL", "OFF")
        .build();

    // Prints instructions for Cargo to link the native library
    println!("cargo:rustc-link-search=native={}/lib", target.display());
    println!("cargo:rustc-link-search=native={}/build/{}", target.display(), if env::var("DEBUG").unwrap() == "true" { "Debug" } else { "Release" });
    println!("cargo:rustc-link-search=native={}/build/shared/src/{}", target.display(), if env::var("DEBUG").unwrap() == "true" { "Debug" } else { "Release" });
    println!("cargo:rustc-link-lib=static=rsllama"); 
    println!("cargo:rustc-link-lib=static=llama"); 

    // Copy dependency lib's to ./target/{releaseType}
    let build_type_dir = if env::var("DEBUG").unwrap() == "true" { "Debug" } else { "Release" };
    let source_pattern = format!("{}/build/{}/*.*", target.display(), build_type_dir);
    for entry in glob(&source_pattern)? {
        let path = entry?;
        let file_name = path.file_name().ok_or("Invalid file name")?;
        let destination_path = Path::new(&("./target/".to_owned() + build_type_dir)).join(file_name);
        eprintln!("Copying {:?} to {:?}", &path, &destination_path);
        let _ = fs::copy(&path, &destination_path);
    }


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

    Ok(())
}