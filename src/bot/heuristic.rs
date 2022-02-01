use std::{cmp::max, fmt::Debug};

use board_game::{
    ai::{minimax::Heuristic, solver::SolverHeuristic, Bot},
    board::Board as BoardTrait,
};
use chess::Color;
use rand::{prelude::IteratorRandom, Rng};

use crate::{
    board::Board,
    move_gen::{Mask, MoveGen},
};

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
pub struct SolverHeuristicSimplified;

impl Heuristic<Board> for SolverHeuristicSimplified {
    type V = i32;

    fn value(&self, board: &Board, depth: u32) -> Self::V {
        SolverHeuristic.value(board, depth).to_i32()
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}

#[derive(Debug, Clone)]
pub struct MaterialHeuristic;

impl Heuristic<Board> for MaterialHeuristic {
    type V = i32;

    fn value(&self, board: &Board, depth: u32) -> Self::V {
        if board.is_done() {
            return SolverHeuristicSimplified.value(board, depth);
        }

        material_eval(board.pieces_to_move().0) as i32
            - material_eval(board.pieces_not_to_move().0) as i32
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}

#[derive(Debug, Clone)]
pub struct AdvancementHeuristic;

impl Heuristic<Board> for AdvancementHeuristic {
    type V = i32;

    fn value(&self, board: &Board, depth: u32) -> Self::V {
        if board.is_done() {
            return SolverHeuristicSimplified.value(board, depth);
        }

        advancement_eval(board.pieces_to_move().0, board.side_to_move()) as i32
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}

pub struct AlwaysPushBot<R: Rng> {
    rng: R,
}

impl<R: Rng> Debug for AlwaysPushBot<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlwaysPushBot")
    }
}

impl<R: Rng + Debug> Bot<Board> for AlwaysPushBot<R> {
    fn select_move(&mut self, board: &Board) -> <Board as BoardTrait>::Move {
        MoveGen::with_mask(board, Mask::Push)
            .choose(&mut self.rng)
            .unwrap_or_else(|| board.random_available_move(&mut self.rng))
    }
}

impl<R: Rng> AlwaysPushBot<R> {
    pub fn new(rng: R) -> Self {
        AlwaysPushBot { rng }
    }
}

pub struct AlwaysCaptureBot<R: Rng> {
    rng: R,
}

impl<R: Rng> Debug for AlwaysCaptureBot<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlwaysCaptureBot")
    }
}

impl<R: Rng + Debug> Bot<Board> for AlwaysCaptureBot<R> {
    fn select_move(&mut self, board: &Board) -> <Board as board_game::board::Board>::Move {
        MoveGen::with_mask(board, Mask::Capture)
            .choose(&mut self.rng)
            .unwrap_or_else(|| board.random_available_move(&mut self.rng))
    }
}

impl<R: Rng> AlwaysCaptureBot<R> {
    pub fn new(rng: R) -> Self {
        AlwaysCaptureBot { rng }
    }
}
