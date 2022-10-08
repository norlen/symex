use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["src/shims.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/shims.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["crus", "libshims.a", "shims.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=shims");
    println!("cargo:rerun-if-changed=src/shims.c");
}
