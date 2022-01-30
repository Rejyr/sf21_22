use std::cmp::max;

use board_game::ai::minimax::Heuristic;
use chess::Color;

use crate::board::Board;

#[allow(non_snake_case)]
pub fn N_fill(mut bb: u64) -> u64 {
    bb |= bb << 8;
    bb |= bb << 16;
    bb |= bb << 32;
    bb
}

#[allow(non_snake_case)]
pub fn S_fill(mut bb: u64) -> u64 {
    bb |= bb >> 8;
    bb |= bb >> 16;
    bb |= bb >> 32;
    bb
}

pub fn advancement_eval(bb: u64, color: Color) -> u32 {
    match color {
        Color::White => S_fill(bb).count_ones(),
        Color::Black => N_fill(bb).count_ones(),
    }
}

pub fn material_eval(bb: u64) -> u32 {
    bb.count_ones()
}

#[derive(Debug, Clone)]
pub struct MaterialHeuristic;

impl Heuristic<Board> for MaterialHeuristic {
    type V = i32;

    fn value(&self, board: &Board, _depth: u32) -> Self::V {
        material_eval(board.pieces_to_move().0) as i32 - material_eval(board.pieces_not_to_move().0) as i32
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}

#[derive(Debug, Clone)]
pub struct AdvancementHeuristic;

impl Heuristic<Board> for AdvancementHeuristic {
    type V = i32;

    fn value(&self, board: &Board, _depth: u32) -> Self::V {
        advancement_eval(board.pieces_to_move().0, board.side_to_move()) as i32
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}
