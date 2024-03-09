use std::{env, io};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=usbip"); 
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen_config().unwrap()
        .generate()
        .expect("Unable to generate the bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}

fn bindgen_config() -> io::Result<bindgen::Builder> {
    let mut config = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_recursively(false)
        .allowlist_item("usbip_.*");

    let reader = BufReader::new(File::open("config/usbip/allowed_functions.txt")?);
    for line in reader.lines() {
        config = config.allowlist_function(line?);
    }

    let reader = BufReader::new(File::open("config/usbip/allowed_types.txt")?);
    for line in reader.lines() {
        config = config.allowlist_type(line?);
    }

    let reader = BufReader::new(File::open("config/usbip/allowed_vars.txt")?);
    for line in reader.lines() {
        config = config.allowlist_var(line?);
    }

    Ok(config.parse_callbacks(Box::new(bindgen::CargoCallbacks::new())))
}
