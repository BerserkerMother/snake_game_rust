use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR: {}", std::env::var("OUT_DIR").unwrap());

    Command::new("gcc")
        .args(&["src/raw.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/raw.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["crus", "libraw.a", "raw.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=raw");
    println!("cargo::rerun-if-changed=src/raw.c");

    let binding = bindgen::Builder::default()
        .header("src/raw.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen gen");

    let gen_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    binding
        .write_to_file(gen_path.join("bindings.rs"))
        .expect("write bindings");
}
