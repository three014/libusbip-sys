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
    let mut builder = bindgen::Builder::default()
        .header(format!("config/{platform}/wrapper.h"))
        .allowlist_recursively(false)
        .allowlist_item("usbip_.*")
        .allowlist_item("USBIP_API");

    for cfg in CONFIG.iter() {
        builder = cfg.call(builder)?;
    }

    Ok(builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new())))
}
