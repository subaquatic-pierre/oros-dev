use std::path::PathBuf;
use std::process::Command;

fn print(val: &str) {
    Command::new("echo")
        .arg(val)
        .spawn()
        .expect("failed to spawn process");
}

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn print_env_vars() {
    for (key, value) in std::env::vars() {
        // let key = key.to_string_lossy();
        // let value = value.to_string_lossy();
        if key.starts_with("CARGO") {
            p!("{key}: {value}");
        }
    }
}

fn main() {
    // set by cargo, build scripts should use this directory for output files
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // set by cargo's artifact dependency feature, see

    // print_env_vars();

    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_OROS_KERNEL_oros-kernel").unwrap());

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();

    // create a BIOS disk image
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());

    p!("UEFI_PATH: {}", uefi_path.display());
    p!("BIOS_PATH: {}", bios_path.display());
}
