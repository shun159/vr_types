use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    Command::new("make")
        .output()
        .expect("Failed to build sandesh modules");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let search_path = Path::new(&dir).join("gen-c/");
    println!("cargo:rustc-link-search=native={}", search_path.display());
    println!("cargo:rustc-link-lib=static=vr_types");
}
