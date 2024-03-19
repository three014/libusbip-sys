use std::path::PathBuf;
use std::env;

use ::bindgen::Builder;

use bindgen::*;
pub mod bindgen;

static UNIX: Config = Config {
    wrapper: "config/linux/wrapper.h",
    link_search: "/usr/lib",
    lib_name: "usbip",
    func_mappings: &[
        GenMap::File {
            path: "config/linux/usbip/allowed_types.txt",
            func: Builder::allowlist_type,
        },
        GenMap::File {
            path: "config/linux/usbip/allowed_vars.txt",
            func: Builder::allowlist_var,
        },
        GenMap::File {
            path: "config/linux/usbip/disallowed_types.txt",
            func: Builder::blocklist_type,
        },
        GenMap::String {
            value: "usbip_.*",
            func: Builder::allowlist_item,
        },
    ],
    output: || PathBuf::from(format!("{}/linux.rs", env::var("OUT_DIR").unwrap())),
};


static _WINDOWS: Config = Config {
    wrapper: r"config\windows\wrapper.h",
    link_search: r"usbip-win2\x64\Debug",
    lib_name: "usbip",
    func_mappings: &[],
    output: || PathBuf::from(format!("{}/windows.rs", env::var("OUT_DIR").unwrap())),
};

fn main() {
    let config = &UNIX;

    println!("cargo:rustc-link-search={}", config.link_search);
    println!("cargo:rustc-link-lib={}", config.lib_name);

    let output = (config.output)();
    let bindings = TryInto::<Builder>::try_into(config)
        .unwrap()
        .generate()
        .expect("Unable to generate the bindings");

    bindings
        .write_to_file(output)
        .expect("Couldn't write bindings!");
}

