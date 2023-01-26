use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rman-schema/schema.fbs");
    let path = env::var_os("OUT_DIR").unwrap();

    Command::new("flatc")
        .args([
            "--rust",
            "-o",
            path.to_str().unwrap(),
            "./rman-schema/schema.fbs",
        ])
        .spawn()
        .expect("Failed to compile flatbuffer schema.");
}
