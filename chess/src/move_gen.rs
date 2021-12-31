#![allow(non_snake_case)]

pub mod pseudo_attacks {
    use crate::shift::*;
    use self::pawn::*;

    pub fn wpawn_atks_pushes(wpawns: u64, empty: u64) -> u64 {
        wpawn_1push(wpawns, empty) | wpawn_2push(wpawns, empty) | wpawn_any_atk(wpawns)
    }

    pub fn bpawn_atks_pushes(bpawns: u64, empty: u64) -> u64 {
        bpawn_1push(bpawns, empty) | bpawn_2push(bpawns, empty) | bpawn_any_atk(bpawns)
    }

    pub fn knight_atks(knights: u64) -> u64 {
        let mut east = E_1(knights);
        let mut west = W_1(knights);
        let mut attacks = N_n(east | west, 2);
        attacks |= S_n(east | west, 2);
        east = E_1(east);
        west = W_1(west);
        attacks |= N_1(east | west);
        attacks |= S_1(east | west);
        attacks
    }

    pub fn king_atks(mut kings: u64) -> u64 {
        let mut attacks = E_1(kings) | W_1(kings);
        kings |= attacks;
        attacks |= N_1(kings) | S_1(kings);
        attacks
    }

    pub mod pawn {
        use crate::{shift::*, board::consts::RANKS};

        #[inline(always)]
        pub fn wpawn_1push(wpawns: u64, empty: u64) -> u64 {
            N_1(wpawns) & empty
        }

        #[inline(always)]
        pub fn wpawn_2push(wpawns: u64, empty: u64) -> u64 {
            let single_pushes = wpawn_1push(wpawns, empty);
            N_1(single_pushes) & empty & RANKS[3]
        }

        #[inline(always)]
        pub fn wpawn_any_atk(wpawns: u64) -> u64 {
            NW_1(wpawns) | NE_1(wpawns)
        }

        #[inline(always)]
        pub fn bpawn_1push(bpawns: u64, empty: u64) -> u64 {
            N_1(bpawns) & empty
        }

        #[inline(always)]
        pub fn bpawn_2push(bpawns: u64, empty: u64) -> u64 {
            let single_pushes = bpawn_1push(bpawns, empty);
            N_1(single_pushes) & empty & RANKS[4]
        }

        #[inline(always)]
        pub fn bpawn_any_atk(bpawns: u64) -> u64 {
            SW_1(bpawns) | SE_1(bpawns)
        }
    }


    pub mod sliding {
        use crate::move_gen::occluded_fill::*;
        use crate::shift::*;

        #[inline(always)]
        pub fn N_atk(ortho: u64, empty: u64) -> u64 {
            N_1(N_ocl(ortho, empty))
        }

        #[inline(always)]
        pub fn S_atk(ortho: u64, empty: u64) -> u64 {
            S_1(S_ocl(ortho, empty))
        }

        #[inline(always)]
        pub fn E_atk(ortho: u64, empty: u64) -> u64 {
            E_1(E_ocl(ortho, empty))
        }

        #[inline(always)]
        pub fn NE_atk(diag: u64, empty: u64) -> u64 {
            NE_1(NE_ocl(diag, empty))
        }

        #[inline(always)]
        pub fn SE_atk(diag: u64, empty: u64) -> u64 {
            SE_1(SE_ocl(diag, empty))
        }

        #[inline(always)]
        pub fn W_atk(ortho: u64, empty: u64) -> u64 {
            W_1(W_ocl(ortho, empty))
        }

        #[inline(always)]
        pub fn NW_atk(diag: u64, empty: u64) -> u64 {
            NW_1(NW_ocl(diag, empty))
        }

        #[inline(always)]
        pub fn SW_atk(diag: u64, empty: u64) -> u64 {
            SW_1(SW_ocl(diag, empty))
        }
    }
}

// https://www.chessprogramming.org/Kogge-Stone_Algorithm
pub mod occluded_fill {
    use crate::board::consts::*;
    use crate::shift::{offsets::*, *};

    #[inline(always)]
    pub fn N_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        pieces |= N_1(empty & pieces);
        empty &= N_1(empty);
        pieces |= N_n(empty & pieces, 2);
        empty &= N_n(empty, 2);
        pieces |= N_n(empty & pieces, 4);
        pieces
    }

    #[inline(always)]
    pub fn S_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        pieces |= S_1(empty & pieces);
        empty &= S_1(empty);
        pieces |= S_n(empty & pieces, 2);
        empty &= S_n(empty, 2);
        pieces |= S_n(empty & pieces, 4);
        pieces
    }

    #[inline(always)]
    pub fn E_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_A;
        pieces |= E_1(empty & pieces) ;
        empty &= E_1(empty);
        pieces |= empty & pieces << (E_W * 2);
        empty &= empty << (E_W * 2);
        pieces |= empty & pieces << (E_W * 4);
        pieces
    }

    #[inline(always)]
    pub fn NE_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_A;
        pieces |= NE_1(empty & pieces) ;
        empty &= NE_1(empty);
        pieces |= empty & pieces << (NE_SW * 2);
        empty &= empty << (NE_SW * 2);
        pieces |= empty & pieces << (NE_SW * 4);
        pieces
    }

    #[inline(always)]
    pub fn SE_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_A;
        pieces |= SE_1(empty & pieces);
        empty &= SE_1(empty);
        pieces |= empty & pieces >> (NW_SE * 2);
        empty &= empty >> (NW_SE * 2);
        pieces |= empty & pieces >> (NW_SE * 4);
        pieces
    }

    #[inline(always)]
    pub fn W_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_H;
        pieces |= W_1(empty & pieces);
        empty &= W_1(empty);
        pieces |= empty & pieces >> (E_W * 2);
        empty &= empty >> (E_W * 2);
        pieces |= empty & pieces >> (E_W * 4);
        pieces
    }

    #[inline(always)]
    pub fn SW_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_H;
        pieces |= SW_1(empty & pieces);
        empty &= SW_1(empty);
        pieces |= empty & pieces >> (NE_SW * 2);
        empty &= empty >> (NE_SW * 2);
        pieces |= empty & pieces >> (NE_SW * 4);
        pieces
    }

    #[inline(always)]
    pub fn NW_ocl(mut pieces: u64, mut empty: u64) -> u64 {
        empty &= NOT_H;
        pieces |= NW_1(empty & pieces);
        empty &= NW_1(empty);
        pieces |= empty & pieces << (NW_SE * 2);
        empty &= empty << (NW_SE * 2);
        pieces |= empty & pieces << (NW_SE * 4);
        pieces
    }
}

#[cfg(test)]
mod test {
    use crate::board::consts::squares::*;
    #[allow(unused_imports)]
    use crate::board::{consts::*, Board, display_bit_board};
    use crate::move_gen::pseudo_attacks::{sliding::*, king_atks, knight_atks, wpawn_atks_pushes};

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

    #[test]
    fn pawn_attack_push() {
        let mut board = Board::default();
        board.white_pawns |= B4;

        assert_eq!(wpawn_atks_pushes(board.white_pawns, board.empty()), (RANKS[2] | RANKS[3] ^ B4) | A5 | B5 | C5);
    }

    #[test]
    fn knight_attack() {
        assert_eq!(knight_atks(C3), B1 | D1 | A2 | E2 | A4 | E4 | B5 | D5);
    }

    #[test]
    fn king_attack() {
        assert_eq!(king_atks(D4), C5 | D5 | E5 | C4 | E4 | C3 | D3 | E3);
    }
}
