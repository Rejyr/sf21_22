#![allow(non_snake_case)]

pub const EMPTY: u64 = 0;
pub const UNIVERSAL: u64 = 0xFFFFFFFFFFFFFFFF;

pub const A_FILE: u64 = 0x0101010101010101;
pub const H_FILE: u64 = 0x8080808080808080;
pub const NOT_A: u64 = 0xfefefefefefefefe;
pub const NOT_H: u64 = 0x7f7f7f7f7f7f7f7f;

pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_8: u64 = 0xFF00000000000000;

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

pub fn shift(b: u64, s: isize) -> u64 {
    if s > 0 {
        b << s
    } else {
        b >> -s
    }
}

#[inline(always)]
pub fn N_1(b: u64) -> u64 {
    b << 8
}

#[inline(always)]
pub fn S_1(b: u64) -> u64 {
    b >> 8
}

#[inline(always)]
pub fn E_1(b: u64) -> u64 {
    (b & NOT_H) << 1
}

#[inline(always)]
pub fn NE_1(b: u64) -> u64 {
    (b & NOT_H) << 9
}

#[inline(always)]
pub fn SE_1(b: u64) -> u64 {
    (b & NOT_H) >> 7
}

#[inline(always)]
pub fn W_1(b: u64) -> u64 {
    (b & NOT_A) >> 1
}

#[inline(always)]
pub fn NW_1(b: u64) -> u64 {
    (b & NOT_A) << 7
}

#[inline(always)]
pub fn SW_1(b: u64) -> u64 {
    (b & NOT_A) >> 9
}

#[inline(always)]
pub fn N_n(b: u64, n: usize) -> u64 {
    b << (8 * n)
}

#[inline(always)]
pub fn S_n(b: u64, n: usize) -> u64 {
    b >> (8 * n)
}

#[inline(always)]
pub fn E_n(b: u64, n: usize) -> u64 {
    (b & NOT_H) << (1 * n)
}

#[inline(always)]
pub fn NE_n(b: u64, n: usize) -> u64 {
    (b & NOT_H) << (9 * n)
}

#[inline(always)]
pub fn SE_n(b: u64, n: usize) -> u64 {
    (b & NOT_H) >> (7 * n)
}

#[inline(always)]
pub fn W_n(b: u64, n: usize) -> u64 {
    (b & NOT_A) >> (1 * n)
}

#[inline(always)]
pub fn NW_n(b: u64, n: usize) -> u64 {
    (b & NOT_A) << (7 * n)
}

#[inline(always)]
pub fn SW_n(b: u64, n: usize) -> u64 {
    (b & NOT_A) >> (9 * n)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::chess::board::shift::*;

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

    #[test]
    fn shift() {
        let b = UNIVERSAL;

        assert_eq!(N_1(b), 0xFFFFFFFFFFFFFF00);
        assert_eq!(NE_1(b), 0xFEFEFEFEFEFEFE00);
        assert_eq!(E_1(b), 0xFEFEFEFEFEFEFEFE);
        assert_eq!(SE_1(b), 0x00FEFEFEFEFEFEFE);
        assert_eq!(S_1(b), 0x00FFFFFFFFFFFFFF);
        assert_eq!(SW_1(b), 0x007F7F7F7F7F7F7F);
        assert_eq!(W_1(b), 0x7F7F7F7F7F7F7F7F);
        assert_eq!(NW_1(b), 0x7F7F7F7F7F7F7F00);
    }
}
