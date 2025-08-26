use std::env;
use std::path::PathBuf;

fn main() {
    println!("build.rs");
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not defined"));
    println!("CARGO_MANIFEST_DIR: {:?}", crate_dir);

    let out_dir = PathBuf::from(env::var("OUT_DIR")
        .expect("OUT_DIR env var is not defined"));
    println!("OUT_DIR: {:?}", out_dir);

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    let cpp_dir = crate_dir.join("cpp");
    if !cpp_dir.exists() {
        std::fs::create_dir_all(&cpp_dir).expect("Unable to create directory: cpp");
    }

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(cpp_dir.join("wv.h"));

    csbindgen::Builder::default()
        .input_extern_file("src/ffi.rs")
        .csharp_dll_name("wv")
        .csharp_namespace("Weave")
        .generate_csharp_file("cs/wv.g.cs")
        .unwrap();
}