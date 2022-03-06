pub mod board;
pub mod bot;
pub mod consts;
pub mod move_gen;

// pub const SIZES: [usize; 2] = [3, 4];
/// All possible sizes of [`Board`](board::Board), from 3 to 8
pub const SIZES: [usize; 6] = [3, 4, 5, 6, 7, 8];

use std::path::PathBuf;

/// Returns the output file path for results
pub fn output_path() -> PathBuf {
    dirs::home_dir().unwrap().join("sf21_22_output")
}

#[cfg(test)]
mod tests;
