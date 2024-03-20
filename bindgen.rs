pub use ::bindgen::Builder;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

pub struct Config {
    pub wrapper: &'static str,
    pub func_mappings: &'static [GenMap],
    pub output: fn() -> PathBuf,
}

impl<'a> TryFrom<&'a Config> for bindgen::Builder {
    type Error = io::Error;

    fn try_from(value: &'a Config) -> Result<Self, Self::Error> {
        let mut builder = bindgen::builder()
            .header(value.wrapper)
            .allowlist_recursively(false);
        for cfg in value.func_mappings.iter() {
            builder = cfg.call(builder)?;
        }
        Ok(builder.parse_callbacks(Box::new(bindgen::CargoCallbacks::new())))
    }
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
