use std::env;
use std::process::Command;
use walkdir::WalkDir;
use std::path::PathBuf;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    match arch.as_str() {
        "x86_64" => {
            // Tell cargo to pass the linker script to the linker..
            println!("cargo:rustc-link-arg=-Tlinker/x86_64.ld");
            // ..and to re-run if it changes.
            println!("cargo:rerun-if-changed=linker/x86_64.ld");
        }
        "aarch64" => {
            // Tell cargo to pass the linker script to the linker..
            println!("cargo:rustc-link-arg=-Tlinker/aarch64.ld");
            // ..and to re-run if it changes.
            println!("cargo:rerun-if-changed=linker/aarch64.ld");
        }
        "riscv64" => {
            // Tell cargo to pass the linker script to the linker..
            println!("cargo:rustc-link-arg=-Tlinker/riscv64.ld");
            // ..and to re-run if it changes.
            println!("cargo:rerun-if-changed=linker/riscv64.ld");
        }
        _ => panic!("Invalid ISA"),
    }

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut objects = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| ext == "asm") {
            let path = entry.path();
            let object = PathBuf::from(&out_dir).join(path.file_stem().unwrap()).with_extension("o");
            Command::new("nasm")
                .args(&["-felf64", "-o", object.to_str().unwrap(), path.to_str().unwrap()])
                .status()
                .expect("Failed to execute NASM");
            objects.push(object);
        }
    }

    let lib_path = format!("{}/libasm.a", out_dir);
    let mut ar_command = Command::new("ar");
    ar_command.arg("crus");
    ar_command.arg(&lib_path);
    for object in &objects {
        ar_command.arg(object);
    }
    ar_command.status().expect("Failed to execute ar");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=asm");
    println!("cargo:rerun-if-changed=asm");
}