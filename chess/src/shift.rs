#![allow(non_snake_case)]

pub mod offsets {
    pub const N_S: usize = 8;
    pub const E_W: usize = 1;
    pub const NE_SW: usize = 9;
    pub const NW_SE: usize = 7;
}

use crate::{board::consts::*, shift::offsets::*};

pub fn shift(b: u64, s: isize) -> u64 {
    if s > 0 {
        b << s
    } else {
        b >> -s
    }
}

#[inline(always)]
pub fn N_1(b: u64) -> u64 {
    b << N_S
}

#[inline(always)]
pub fn S_1(b: u64) -> u64 {
    b >> N_S
}

#[inline(always)]
pub fn E_1(b: u64) -> u64 {
    (b & NOT_H) << E_W
}

#[inline(always)]
pub fn NE_1(b: u64) -> u64 {
    (b & NOT_H) << NE_SW
}

#[inline(always)]
pub fn SE_1(b: u64) -> u64 {
    (b & NOT_H) >> NW_SE
}

#[inline(always)]
pub fn W_1(b: u64) -> u64 {
    (b & NOT_A) >> E_W
}

#[inline(always)]
pub fn NW_1(b: u64) -> u64 {
    (b & NOT_A) << NW_SE
}

#[inline(always)]
pub fn SW_1(b: u64) -> u64 {
    (b & NOT_A) >> NE_SW
}

#[inline(always)]
pub fn N_n(b: u64, n: usize) -> u64 {
    b << (N_S * n)
}

#[inline(always)]
pub fn S_n(b: u64, n: usize) -> u64 {
    b >> (N_S * n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shift() {
        let b = UNIVERSAL;

        assert_eq!(N_1(b), NOT_1);
        assert_eq!(NE_1(b), NOT_1 & NOT_A);
        assert_eq!(E_1(b), NOT_A);
        assert_eq!(SE_1(b), NOT_A & NOT_8);
        assert_eq!(S_1(b), NOT_8);
        assert_eq!(SW_1(b), NOT_8 & NOT_H);
        assert_eq!(W_1(b), NOT_H);
        assert_eq!(NW_1(b), NOT_H & NOT_1);
    }
}
