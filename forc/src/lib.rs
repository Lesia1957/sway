pub mod cli;
pub mod ops;
pub mod utils;

#[cfg(feature = "test")]
pub mod test {
    pub use crate::cli::BuildCommand;
    pub use crate::ops::{forc_build, forc_check};
}

pub use regex::Regex;

#[cfg(feature = "util")]
pub mod util {
    pub use sway_utils::constants;
}

pub extern crate paste;
pub use serial_test;
