#![allow(non_snake_case)]
use crate::{board::consts::*, shift::N_1};

pub mod consts;


// Little-Endian File-Rank mapping
#[derive(Debug, Clone)]
pub struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_kings: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_kings: u64,
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

pub fn display_bit_board(b: u64) {
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

        display_bit_board(b.white_pawns);
    }
}
