//! Heuristics ([`Advancement`](AdvancementHeuristic), [`Material`](MaterialHeuristic))
//! and the heuristic bots ([`AlwaysPush`](AlwaysPushBot), [`AlwaysCapture`](AlwaysCaptureBot))

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

/// North [pawn fill](https://www.chessprogramming.org/Pawn_Fills)
/// using parallel prefix [Kogge-Stone routines](https://www.chessprogramming.org/Kogge-Stone_Algorithm)
#[allow(non_snake_case)]
pub fn N_fill(mut bb: u64) -> u64 {
    bb |= bb << 8;
    bb |= bb << 16;
    bb |= bb << 32;
    bb
}

/// South [pawn fill](https://www.chessprogramming.org/Pawn_Fills)
/// using parallel prefix [Kogge-Stone routines](https://www.chessprogramming.org/Kogge-Stone_Algorithm)
#[allow(non_snake_case)]
pub fn S_fill(mut bb: u64) -> u64 {
    bb |= bb >> 8;
    bb |= bb >> 16;
    bb |= bb >> 32;
    bb
}

/// Returns an evaluation of how far the pawns are by counting the [rear-fill](https://www.chessprogramming.org/Pawn_Fills)
pub fn advancement_eval(bb: u64, color: Color) -> u32 {
    match color {
        Color::White => S_fill(bb).count_ones(),
        Color::Black => N_fill(bb).count_ones(),
    }
}

/// Returns an evaluation of the amount of pawns by counting the [population count](https://www.chessprogramming.org/Population_Count)
pub fn material_eval(bb: u64) -> u32 {
    bb.count_ones()
}

#[derive(Debug, Clone)]
/// A simplified [`SolverHeuristic`](SolverHeuristic) by converting to i32
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
/// Returns an evaluation of \# player's pawns - \# opponent's pawns
pub struct MaterialHeuristic;

impl Heuristic<Board> for MaterialHeuristic {
    type V = i32;

    fn value(&self, board: &Board, depth: u32) -> Self::V {
        // if the board is done, it's infinity for winning, negative infinity for losing
        if board.is_done() {
            return SolverHeuristicSimplified.value(board, depth);
        }

        // return the difference between the amount of the player's pawns and the amount of the opponent's pawns
        material_eval(board.pieces_to_move().0) as i32
            - material_eval(board.pieces_not_to_move().0) as i32
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}

#[derive(Debug, Clone)]
/// Returns an evaluation of how far the player's pawns are
pub struct AdvancementHeuristic;

impl Heuristic<Board> for AdvancementHeuristic {
    type V = i32;

    fn value(&self, board: &Board, depth: u32) -> Self::V {
        // if the board is done, it's infinity for winning, negative infinity for losing
        if board.is_done() {
            return SolverHeuristicSimplified.value(board, depth);
        }

        // return how far the pawns are
        advancement_eval(board.pieces_to_move().0, board.side_to_move()) as i32
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, std::cmp::Ordering) {
        (max(old, new), new.cmp(&old))
    }
}

/// The [`AlwaysPush`](AlwaysPushBot) bot. It always pushes a pawn or chooses a random move
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
    /// Creates a new [`AlwaysPushBot`](AlwaysPushBot)
    pub fn new(rng: R) -> Self {
        AlwaysPushBot { rng }
    }
}

/// The [`AlwaysCapture`](AlwaysCaptureBot) bot. It always captures a pawn or chooses a random move
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
    /// Creates a new [`AlwaysCaptureBot`](AlwaysCaptureBot)
    pub fn new(rng: R) -> Self {
        AlwaysCaptureBot { rng }
    }
}
