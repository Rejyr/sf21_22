pub mod board;
pub mod bot;
pub mod consts;
pub mod move_gen;

pub const SIZES: [usize; 2] = [3, 4];
// pub const SIZES: [usize; 6] = [3, 4, 5, 6, 7, 8];

use std::path::PathBuf;

pub fn output_path() -> PathBuf {
    dirs::home_dir().unwrap().join("sf21_22_output")
}

#[cfg(test)]
mod tests;
