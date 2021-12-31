#![allow(non_snake_case)]

pub mod attacks {
    pub mod sliding {
        use crate::move_gen::occluded_fill::*;
        use crate::shift::*;

        pub fn N_atk(ortho: u64, empty: u64) -> u64 {
            N_1(N_ocl(ortho, empty))
        }

        pub fn S_atk(ortho: u64, empty: u64) -> u64 {
            S_1(S_ocl(ortho, empty))
        }

        pub fn E_atk(ortho: u64, empty: u64) -> u64 {
            E_1(E_ocl(ortho, empty))
        }

        pub fn NE_atk(diag: u64, empty: u64) -> u64 {
            NE_1(NE_ocl(diag, empty))
        }

        pub fn SE_atk(diag: u64, empty: u64) -> u64 {
            SE_1(SE_ocl(diag, empty))
        }

        pub fn W_atk(ortho: u64, empty: u64) -> u64 {
            W_1(W_ocl(ortho, empty))
        }

        pub fn NW_atk(diag: u64, empty: u64) -> u64 {
            NW_1(NW_ocl(diag, empty))
        }

        pub fn SW_atk(diag: u64, empty: u64) -> u64 {
            SW_1(SW_ocl(diag, empty))
        }
    }
}

// https://www.chessprogramming.org/Kogge-Stone_Algorithm
pub mod occluded_fill {
    use crate::board::consts::*;
    use crate::shift::offsets::*;

    pub fn N_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        pieces |= empty & pieces << N_S;
        empty &= empty << N_S;
        pieces |= empty & pieces << (N_S * 2);
        empty &= empty << (N_S * 2);
        pieces |= empty & pieces << (N_S * 4);
        pieces
    }

    pub fn S_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        pieces |= empty & pieces >> N_S;
        empty &= empty >> N_S;
        pieces |= empty & pieces >> (N_S * 2);
        empty &= empty >> (N_S * 2);
        pieces |= empty & pieces >> (N_S * 4);
        pieces
    }

    pub fn E_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_A;
        pieces |= empty & pieces << E_W;
        empty &= empty << E_W;
        pieces |= empty & pieces << (E_W * 2);
        empty &= empty << (E_W * 2);
        pieces |= empty & pieces <<(E_W * 4);
        pieces
    }

    pub fn NE_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_A;
        pieces |= empty & pieces << NE_SW;
        empty &= empty << NE_SW;
        pieces |= empty & pieces <<(NE_SW * 2);
        empty &= empty <<(NE_SW * 2);
        pieces |= empty & pieces <<(NE_SW * 4);
        pieces
    }

    pub fn SE_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_A;
        pieces |= empty & pieces >> NW_SE;
        empty &= empty >> NW_SE;
        pieces |= empty & pieces >>(NW_SE * 2);
        empty &= empty >>(NW_SE * 2);
        pieces |= empty & pieces >>(NW_SE * 4);
        pieces
    }

    pub fn W_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_H;
        pieces |= empty & pieces >> E_W;
        empty &= empty >> E_W;
        pieces |= empty & pieces >> (E_W * 2);
        empty &= empty >> (E_W * 2);
        pieces |= empty & pieces >>(E_W * 4);
        pieces
    }

    pub fn SW_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_H;
        pieces |= empty & pieces >> NE_SW;
        empty &= empty >> NE_SW;
        pieces |= empty & pieces >>(NE_SW * 2);
        empty &= empty >>(NE_SW * 2);
        pieces |= empty & pieces >>(NE_SW * 4);
        pieces
    }

    pub fn NW_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_H;
        pieces |= empty & pieces << NW_SE;
        empty &= empty << NW_SE;
        pieces |= empty & pieces <<(NW_SE * 2);
        empty &= empty <<(NW_SE * 2);
        pieces |= empty & pieces <<(NW_SE * 4);
        pieces
    }
}

#[cfg(test)]
mod test {
    use crate::board::{consts::*, display_bit_board};
    use crate::board::consts::squares::*;
    use crate::move_gen::attacks::sliding::*;

    #[test]
    fn sliding_attack() {
        let empty = UNIVERSAL;
        let ns_attack = N_atk(D4, empty) | S_atk(D4, empty);
        let ew_attack = E_atk(D4, empty) | W_atk(D4, empty);
        let ne_sw_attack = NE_atk(D4, empty) | SW_atk(D4, empty);
        let nw_se_attack = NW_atk(D4, empty) | SE_atk(D4, empty);

        assert_eq!(ns_attack, FILES[3] ^ D4);
        assert_eq!(ew_attack, RANKS[3] ^ D4);
        assert_eq!(ne_sw_attack, A1_H8_DIAG ^ D4);
        assert_eq!(nw_se_attack, (G1 | F2 | E3 | D4 | C5 | B6 | A7) ^ D4);
    }
}
