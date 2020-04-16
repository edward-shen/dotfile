use std::collections::HashSet;

#[derive(Debug)]
pub enum DotfileError {
    ConfigParse(serde_yaml::Error),
    Io(std::io::Error),
    AmbiguousOrUnknownItems(HashSet<String>, HashSet<String>),
}

/// Generates a from implementation from the specified type to the provided
/// dotfile error.
macro_rules! from_error {
    ($from:ty, $to:ident) => {
        impl From<$from> for DotfileError {
            fn from(e: $from) -> Self {
                Self::$to(e)
            }
        }
    };
}

from_error!(serde_yaml::Error, ConfigParse);
from_error!(std::io::Error, Io);
