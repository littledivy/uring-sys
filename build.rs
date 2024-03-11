use std::env;
use std::fs;
use std::path::*;
use std::process::Command;

use cc::Build;

fn main() {
    let project = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let liburing = project.join("liburing");

    let include = liburing.join("src/include");

    let src = liburing.join("src");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap();

    // liburing
    let cmd = Command::new("make")
        .current_dir(&liburing)
        .output()
        .unwrap();
    if !cmd.status.success() {
        panic!(
            "make failed: {}",
            String::from_utf8(cmd.stderr).unwrap_or_default()
        );
    }
    println!(
        "cargo:rustc-link-search=native={}",
        liburing.join("src").display()
    );
    println!("cargo:rustc-link-lib=static=uring-ffi");

    let bindings = bindgen::Builder::default()
        .header(include.join("liburing.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_suffix("")
        .allowlist_file("^(.*liburing.h)$")
        .allowlist_file("^(.*io_uring.h)$")
        .rustified_enum("io_uring_op") // Not used in C code; safe to rustify
        .rustified_non_exhaustive_enum("io_uring_op")
        // C++ atomics.
        .clang_arg("-xc++")
        .trust_clang_mangling(false)
        .generate()
        .unwrap();

    let out_path = out_dir.join("bindings.rs");
    bindings.write_to_file(out_path).unwrap();

    println!("cargo:include={}", include.display());
}
