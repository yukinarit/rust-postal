extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = pkg_config::probe_library("libpostal").expect("libpostal pkg-config not found");

    println!("cargo:rustc-link-lib=dylib=postal");

    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .clang_args(
            lib.include_paths
                .iter()
                .map(|p| format!("-I{}", p.to_string_lossy())),
        )
        .header("wrapper.h")
        .derive_debug(true)
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
