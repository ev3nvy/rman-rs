use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rman-schema/schema.fbs");
    let path = env::var_os("OUT_DIR").unwrap();

    #[cfg(target_os = "windows")]
    let flatc = "./flatc/Windows.flatc.binary/flatc.exe";
    #[cfg(target_os = "macos")]
    let flatc = "./flatc/Mac.flatc.binary/flatc";
    #[cfg(target_os = "linux")]
    let flatc = "./flatc/Linux.flatc.binary.clang++-12/flatc";
    Command::new(flatc)
        .args([
            "--rust",
            "-o",
            path.to_str().unwrap(),
            "./rman-schema/schema.fbs",
        ])
        .spawn()
        .expect("Failed to compile flatbuffer schema.");
}
