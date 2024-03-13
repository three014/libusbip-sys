use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{env, io};

fn main() {
    #[cfg(target_family = "unix")]
    {
        println!("cargo:rustc-link-search=/usr/lib");
        println!("cargo:rustc-link-lib=usbip");
        println!("cargo:rerun-if-changed=config/linux/wrapper.h");
    }

    let bindings = bindgen_config()
        .unwrap()
        .generate()
        .expect("Unable to generate the bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("{}.rs", get_sys())))
        .expect("Couldn't write bindings!");
}

const fn get_sys() -> &'static str {
    if cfg!(unix) {
        "linux"
    } else if cfg!(windows) {
        "windows"
    } else {
        panic!("Unsupported OS!")
    }
}

fn bindgen_config() -> io::Result<bindgen::Builder> {
    let platform = get_sys();
    let mut config = bindgen::Builder::default()
        .header(format!("config/{platform}/wrapper.h"))
        .allowlist_recursively(false)
        .allowlist_item("usbip_.*");

    let reader = BufReader::new(File::open(format!(
        "config/{platform}/usbip/allowed_functions.txt"
    ))?);
    for line in reader.lines() {
        config = config.allowlist_function(line?);
    }

    let reader = BufReader::new(File::open(format!(
        "config/{platform}/usbip/allowed_types.txt"
    ))?);
    for line in reader.lines() {
        config = config.allowlist_type(line?);
    }

    let reader = BufReader::new(File::open(format!(
        "config/{platform}/usbip/allowed_vars.txt"
    ))?);
    for line in reader.lines() {
        config = config.allowlist_var(line?);
    }

    let reader = BufReader::new(File::open(format!(
        "config/{platform}/usbip/disallowed_types.txt"
    ))?);
    for line in reader.lines() {
        config = config.blocklist_type(line?);
    }

    Ok(config.parse_callbacks(Box::new(bindgen::CargoCallbacks::new())))
}
