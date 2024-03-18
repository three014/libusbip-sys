use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{env, io};

struct ConfigMap {
    file: &'static str,
    func: fn(bindgen::Builder, String) -> bindgen::Builder,
}

impl ConfigMap {
    pub fn call(&self, mut b: bindgen::Builder) -> io::Result<bindgen::Builder> {
        let reader = BufReader::new(File::open(&self.file)?);
        for line in reader.lines() {
            b = (self.func)(b, line?);
        }

        Ok(b)
    }
}

static CONFIG: [ConfigMap; 4] = [
    ConfigMap {
        file: "config/linux/usbip/allowed_functions.txt",
        func: bindgen::Builder::allowlist_function,
    },
    ConfigMap {
        file: "config/linux/usbip/allowed_types.txt",
        func: bindgen::Builder::allowlist_type,
    },
    ConfigMap {
        file: "config/linux/usbip/allowed_vars.txt",
        func: bindgen::Builder::allowlist_var,
    },
    ConfigMap {
        file: "config/linux/usbip/disallowed_types.txt",
        func: bindgen::Builder::blocklist_type,
    },
];

fn main() {
    #[cfg(target_family = "unix")]
    {
        println!("cargo:rustc-link-search=/usr/lib");
    }

    #[cfg(target_family = "windows")]
    {
        println!("cargo:rustc-link-search={}", r"usbip-win2\x64\Debug");
    }

    let wrapper: PathBuf = ["config", get_sys(), "wrapper.h"].iter().collect();

    println!("cargo:rustc-link-lib=usbip");
    println!("cargo:rerun-if-changed={}", wrapper.display());

    let bindings = bindgen_config(wrapper)
        .unwrap()
        .generate()
        .expect("Unable to generate the bindings");

    let output = PathBuf::from(format!("{}/{}.rs", env::var("OUT_DIR").unwrap(), get_sys()));

    bindings
        .write_to_file(output)
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

/// Creates the Bindgen configuration for
/// the usbip library. In Linux we use
/// libusbip (C), while in Windows we use
/// usbip-win2 (C++).
fn bindgen_config(wrapper: PathBuf) -> io::Result<bindgen::Builder> {
    let mut builder = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .allowlist_recursively(false);

    if cfg!(windows) {
        builder = builder
            .opaque_type("std::.*")
            .clang_arg("-Iusbip-win2\\userspace\\")
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg(r"-std=c++20")
            .allowlist_item("USBIP_API");
    } else {
        builder = builder.allowlist_item("usbip_.*");
    }

    for cfg in CONFIG.iter() {
        builder = cfg.call(builder)?;
    }

    Ok(builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new())))
}
