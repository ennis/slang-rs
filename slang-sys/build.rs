extern crate bindgen;
extern crate cc;

use std::fs;
use std::{env, error::Error, path::PathBuf};
use std::path::Path;


fn write_target_link_libraries(slang_install_dir: &Path) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("cannot determine target OS");
    let target_bitwidth = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("cannot determine bit width of target");

    let bin_subdir = match &*target_os {
        "linux" | "android" => {
            println!("cargo:rustc-link-lib=dylib=slang");
            println!("cargo:rustc-link-lib=dylib=slang-glslang");
            if target_bitwidth == "64" {
                "linux-x64"
            } else {
                "linux-x86"
            }
        },
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=slang");
            if target_bitwidth == "64" {
                "windows-x64"
            } else {
                "windows-x86"
            }
        }
        tos => panic!("Unsupported or unknown target: {:?}", tos)
    };

    let bin_dir = slang_install_dir.join(format!("bin/{}/release", bin_subdir));

    // quick sanity check
    if !bin_dir.is_dir() {
        panic!("

The inferred target-specific `bin` subdirectory (bin/{}/release) does not exist in \
the slang installation directory. Your slang installation may not match the target \
you are building `slang-sys` for.

", bin_subdir)
    }

    println!("cargo:rustc-link-search=native={}", bin_dir.display())
}

fn main() {
    // find slang libraries and headers via the SLANG_DIR env var
    let slang_dir = env::var("SLANG_DIR");

    let slang_dir = if let Ok(v) = slang_dir { PathBuf::from(v) } else {
        panic!("

The `SLANG_DIR` environment variable was not set and this `-sys` crate cannot find \
the path to a slang installation without it. Please set `SLANG_DIR` to the \
directory of the slang installation. This directory should contain `slang.h` and a \
`bin` subdirectory.

")
    };

    let slang_header = slang_dir.join("slang.h");

    if !slang_header.is_file() {
        panic!("

The `slang.h` header file was not found within the directory specified in \
`SLANG_DIR`. Please specify a valid installation directory.

Searched for: {}

", slang_header.display())
    }

    write_target_link_libraries(&slang_dir);

    // bindgen our functions
    let bindings = {
        let mut builder = bindgen::Builder::default();
        // The input header we would like to generate
        builder = builder
            .header(slang_header.to_str().unwrap().to_string())
            .clang_arg("-v")
            .clang_arg("-xc++")
            .clang_arg("-std=c++14")
            .derive_copy(true)
            .with_codegen_config(
                bindgen::CodegenConfig::TYPES
                    | bindgen::CodegenConfig::FUNCTIONS
                    | bindgen::CodegenConfig::VARS,
            );

        builder.generate()
    }
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
