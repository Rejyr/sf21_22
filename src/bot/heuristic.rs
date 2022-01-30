use std::{cmp::max, fmt::Debug};

use board_game::ai::{minimax::Heuristic, Bot};
use chess::Color;
use internal_iterator::IteratorExt;
use rand::{Rng, prelude::IteratorRandom};

use crate::{board::Board, move_gen::{MoveGen, Mask}};

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

#[derive(Debug)]
pub struct PawnPusherBot<R: Rng> {
    rng: R,
}

impl<R: Rng + Debug> Bot<Board> for PawnPusherBot<R> {
    fn select_move(&mut self, board: &Board) -> <Board as board_game::board::Board>::Move {
        MoveGen::with_mask(board, Mask::Push).choose(&mut self.rng).unwrap()
    }
}

#[derive(Debug)]
pub struct AlwaysCaptureBot<R: Rng> {
    rng: R,
}

impl<R: Rng + Debug> Bot<Board> for AlwaysCaptureBot<R> {
    fn select_move(&mut self, board: &Board) -> <Board as board_game::board::Board>::Move {
        MoveGen::with_mask(board, Mask::Capture).choose(&mut self.rng).unwrap()
    }
}
