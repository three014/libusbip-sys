use std::path::PathBuf;
use std::{env, io};

use config::*;

pub mod config {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::path::PathBuf;

    pub struct Config {
        pub wrapper: &'static str,
        pub link_search: &'static str,
        pub lib_name: &'static str,
        pub func_mappings: &'static [GenMap],
        pub output: fn() -> PathBuf,
    }

    pub enum GenMap {
        File {
            path: &'static str,
            func: fn(bindgen::Builder, String) -> bindgen::Builder,
        },
        String {
            value: &'static str,
            func: fn(bindgen::Builder, String) -> bindgen::Builder,
        },
    }

    impl GenMap {
        pub fn call(&self, mut b: bindgen::Builder) -> io::Result<bindgen::Builder> {
            match self {
                GenMap::File { path, func } => {
                    let reader = BufReader::new(File::open(path)?);
                    for line in reader.lines() {
                        b = func(b, line?);
                    }
                }
                GenMap::String { value, func } => {
                    b = func(b, value.to_string());
                }
            }

            Ok(b)
        }
    }
}

static UNIX: Config = Config {
    wrapper: "config/linux/wrapper.h",
    link_search: "/usr/lib",
    lib_name: "usbip",
    func_mappings: &[
        GenMap::File {
            path: "config/linux/usbip/allowed_types.txt",
            func: bindgen::Builder::allowlist_type,
        },
        GenMap::File {
            path: "config/linux/usbip/allowed_vars.txt",
            func: bindgen::Builder::allowlist_var,
        },
        GenMap::File {
            path: "config/linux/usbip/disallowed_types.txt",
            func: bindgen::Builder::blocklist_type,
        },
        GenMap::String {
            value: "usbip_.*",
            func: bindgen::Builder::allowlist_item,
        },
    ],
    output: || PathBuf::from(format!("{}/linux.rs", env::var("OUT_DIR").unwrap())),
};

static WINDOWS: Config = Config {
    wrapper: r"config\windows\wrapper.h",
    link_search: r"usbip-win2\x64\Debug",
    lib_name: "usbip",
    func_mappings: &[],
    output: || PathBuf::from(format!("{}/windows.rs", env::var("OUT_DIR").unwrap())),
};

fn get_config() -> &'static Config {
    if cfg!(unix) {
        &UNIX
    } else if cfg!(windows) {
        &WINDOWS
    } else {
        panic!("Unsupported OS!")
    }
}

fn main() {
    let config = get_config();

    println!("cargo:rustc-link-search={}", config.link_search);
    println!("cargo:rustc-link-lib={}", config.lib_name);
    println!("cargo:rerun-if-changed={}", config.wrapper);

    let bindings = bindgen_config(config)
        .unwrap()
        .generate()
        .expect("Unable to generate the bindings");

    let output = (config.output)();

    bindings
        .write_to_file(output)
        .expect("Couldn't write bindings!");
}

/// Creates the Bindgen configuration for
/// the usbip library. In Linux we use
/// libusbip (C), while in Windows we use
/// usbip-win2 (C++).
fn bindgen_config(config: &Config) -> io::Result<bindgen::Builder> {
    let mut builder = bindgen::Builder::default()
        .header(config.wrapper)
        .allowlist_recursively(false);

    if cfg!(windows) {
        builder = builder
            .opaque_type("std::.*")
            .clang_arg(r"-Iusbip-win2\userspace\")
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg(r"-std=c++20")
            .allowlist_item("USBIP_API");
    }

    for cfg in config.func_mappings.iter() {
        builder = cfg.call(builder)?;
    }

    Ok(builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new())))
}
