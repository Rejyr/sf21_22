#![allow(non_snake_case)]
use crate::{board::consts::*, shift::N_1};

pub mod consts;

// Little-Endian File-Rank mapping
#[derive(Debug, Clone)]
pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_kings: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_kings: u64,
}

impl Default for Board {
    #[allow(clippy::unusual_byte_groupings)]
    fn default() -> Self {
        Self {
            white_pawns: 0x000000000000_FF_00,
            white_knights: 0x00000000000000_42,
            white_bishops: 0x00000000000000_24,
            white_rooks: 0x00000000000000_81,
            white_queens: 0x00000000000000_08,
            white_kings: 0x00000000000000_10,
            black_pawns: 0x00_FF_000000000000,
            black_knights: 0x42_00000000000000,
            black_bishops: 0x24_00000000000000,
            black_rooks: 0x81_00000000000000,
            black_queens: 0x08_00000000000000,
            black_kings: 0x10_00000000000000,
        }
    }
}

impl Board {
    pub fn white_pieces(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_kings
    }

    pub fn black_pieces(&self) -> u64 {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_kings
    }

    pub fn occupied(&self) -> u64 {
        self.white_pieces() | self.black_pieces()
    }

    pub fn empty(&self) -> u64 {
        !self.occupied()
    }
}

pub fn display_bb(b: u64) {
    let mut b = b;
    // get rank 8 and shift up, otherwise displays displays lower ranks higher up (flipped)
    for i in 0..8 {
        let rank = ((b & RANK_8) >> 56) as u8; // shift down 56 to fit into u8
        println!("{:08b}|{}", rank.reverse_bits(), 8 - i); // reverse bits for little to big (for bitboard representation) endianness
        b = N_1(b);
    }
    println!("--------*");
    println!("abcdefgh");
    println!("01234567");
}

// De Bruijn mulutiplication with separated LS1B
// https://www.chessprogramming.org/BitScan
pub fn bit_scan_forward(b: u64) -> u8 {
    const DEBRUIJN_ARR: [u8; 64] = [
        0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10,
        45, 25, 39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
    ];
    const DEBRUIJN_NUM: u64 = 0x03f79d71b4cb0a89;

    assert_ne!(b, 0);
    DEBRUIJN_ARR[((b ^ b.wrapping_sub(1)).wrapping_mul(DEBRUIJN_NUM) >> 58) as usize]
}

pub fn serialize_bb(mut b: u64) -> Vec<u8> {
    let mut idxs = vec![];
    while b != 0 {
        idxs.push(bit_scan_forward(b));

        b &= b.wrapping_sub(1); // reset LS1B
    }
    idxs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_board() {
        let b = Board::default();

        // display_bit_board(board.occupied());
        assert_eq!(b.white_pieces(), 0x000000000000FFFF);
        assert_eq!(b.black_pieces(), 0xFFFF000000000000);
        assert_eq!(b.occupied(), 0xFFFF00000000FFFF);
    }

    #[test]
    fn display_bitboards() {
        let b = Board::default();

        display_bb(b.white_pawns);
    }

    #[test]
    fn bitboard_serialization() {
        assert_eq!(
            serialize_bb(UNIVERSAL),
            (0..64).into_iter().collect::<Vec<_>>()
        )
    }
}
