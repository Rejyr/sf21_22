use squares::*;

pub const EMPTY: u64 = 0;
pub const UNIVERSAL: u64 = 0xFFFFFFFFFFFFFFFF;

// a-h
pub const FILES: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];
pub const A_FILE: u64 = FILES[0];
pub const H_FILE: u64 = FILES[7];
pub const NOT_A: u64 = !A_FILE;
pub const NOT_H: u64 = !H_FILE;

// ranks 1-8
pub const RANKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];
pub const RANK_1: u64 = RANKS[0];
pub const RANK_8: u64 = RANKS[7];
pub const NOT_1: u64 = !RANK_1;
pub const NOT_8: u64 = !RANK_8;

pub const A1_H8_DIAG: u64 = A1 | B2 | C3 | D4 | E5 | F6 | G7 | H8;
pub const A8_H1_DIAG: u64 = A8 | B7 | C6 | D5 | E4 | F3 | G2 | H1;
pub const A7_G1_DIAG: u64 = G1 | F2 | E3 | D4 | C5 | B6 | A7;

pub const fn SQUARE(n: usize) -> u64 {
    assert!(n < 64, "Out of bounds index (index must be less than 64)");
    1 << n
}

pub mod squares;
