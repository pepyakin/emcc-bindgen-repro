extern crate bindgen;

use std::path::PathBuf;
use std::process::Command;
use std::env;

fn gen_bindings() {
    let bindings = bindgen::Builder::default()
        .header("adder.h")
        // .trust_clang_mangling(false)
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    gen_bindings();

    let current_dir = env::current_dir().unwrap();

    let _ = Command::new("emcc")
        .args(&["-o", "adder.bc", "adder.c"])
        .status()
        .expect("failed to compile adder.c");
    let _ = Command::new("emar")
        .args(&["cr", "libadder.a", "adder.bc"])
        .status()
        .expect("failed to archive libadder.a");

    println!("cargo:rustc-link-search=native={}", current_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=adder");
}
