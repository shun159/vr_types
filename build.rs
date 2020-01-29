use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main(){
    Command::new("./sandesh_idl")
        .args(&["--gen", "c", "priv/vr.sandesh"])
        .output()
        .expect("Failed to generate Sandesh IDL");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_dir_str = out_dir.to_str().unwrap();

    cc::Build::new()
        .flag("-std=c99")
        .flag("-g")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-strict-aliasing")
        .flag("-Wno-sign-compare")
        .flag("-Wno-format")
        .include(".")
        .include("sandesh/library/c/")
        .file("sandesh/library/c/protocol/thrift_binary_protocol.c")
        .file("sandesh/library/c/protocol/thrift_protocol.c")
        .file("sandesh/library/c/protocol/thrift_xml_protocol.c")
        .file("sandesh/library/c/transport/thrift_fake_transport.c")
        .file("sandesh/library/c/transport/thrift_file_transport.c")
        .file("sandesh/library/c/transport/thrift_memory_buffer.c")
        .file("sandesh/library/c/sandesh.c")
        .file("gen-c/vr_types.c")
        .out_dir(out_dir_str)
        .compile("vr_types");

    println!("cargo:rustc-link-search=native={}", out_dir_str);
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("sandesh/library/c").display());
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("sandesh/library/c/protocol").display());
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("sandesh/library/c/transport").display());
    println!("cargo:rustc-link-lib=vr_types");
}
