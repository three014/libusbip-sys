use bindgen::*;
pub mod bindgen;

static UNIX: Lib = Lib {
    link_search: "/usr/lib",
    lib_name: "usbip",
    config: Config {
        wrapper: "config/unix/wrapper.h",
        func_mappings: &[
            GenMap::File {
                path: "config/unix/usbip/allowed_types.txt",
                func: Builder::allowlist_type,
            },
            GenMap::File {
                path: "config/unix/usbip/allowed_vars.txt",
                func: Builder::allowlist_var,
            },
            GenMap::File {
                path: "config/unix/usbip/disallowed_types.txt",
                func: Builder::blocklist_type,
            },
            GenMap::String {
                value: "usbip_.*",
                func: Builder::allowlist_item,
            },
        ],
        output: || {
            std::path::PathBuf::from(format!("{}/unix.rs", std::env::var("OUT_DIR").unwrap()))
        },
    },
};

pub struct Lib {
    link_search: &'static str,
    lib_name: &'static str,
    config: Config,
}

fn main() {
    let lib = &UNIX;

    println!("cargo:rustc-link-search={}", lib.link_search);
    println!("cargo:rustc-link-lib={}", lib.lib_name);

    let output = (lib.config.output)();
    let bindings = TryInto::<Builder>::try_into(&lib.config)
        .unwrap()
        .generate()
        .expect("Unable to generate the bindings");

    bindings
        .write_to_file(output)
        .expect("Couldn't write bindings!");
}
