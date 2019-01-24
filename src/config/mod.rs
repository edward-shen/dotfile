use std::fs::write;
use std::path::PathBuf;

use serde::Serialize;

pub mod global;
pub mod local;

pub trait Writable {
    /// Directly write to the file, overwriting the currently existing file.
    fn write_to_file(&self, file_path: &PathBuf)
    where
        Self: Serialize,
    {
        write(file_path, toml::to_string_pretty(self).unwrap()).expect("Could not write to file!");
    }
}
