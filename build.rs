extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    // use pkg_config to search for xenstore.pc config file
    // disable cargo metadata, we need to configure rustc manually
    // do not use .statik(), since this feature is buggy due to
    // https://github.com/rust-lang/pkg-config-rs/issues/102
    let xenstore = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("xenstore")
        .expect("Failed to locate xenstore library");

    // add link paths
    for path in &xenstore.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // manually specify xentoolcore, since we have no way to retrieve the "Requires.private" xenstore.pc field
    // from the Library struct returned by pkg_config
    // and we don't use .statik(), see message above
    if cfg!(feature = "static") {
        println!("cargo:rustc-link-lib=static=xenstore");
        println!("cargo:rustc-link-lib=static=xentoolcore");
    } else {
        println!("cargo:rustc-link-lib=xenstore");
    }

    // include paths will be added as additional clang args to Bindgen Builder
    let include_args: Vec<String> = xenstore
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.to_string_lossy()))
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // additional include paths
        .clang_args(include_args)
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
