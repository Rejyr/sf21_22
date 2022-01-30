use std::{fmt::Display, cmp::Ordering};

use chess::{Square, BitBoard, get_pawn_quiets, get_pawn_attacks};

use crate::{board::Board, consts::EMPTY_BB};

pub struct SquareAndBitBoard {
    sq: Square,
    bb: BitBoard,
}

pub enum Mask {
    None,
    Capture,
    Push,
}

pub struct MoveGen {
    moves: Vec<SquareAndBitBoard>,
    index: usize,
}

// trimmed ChessMove from chess
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug, Hash)]
pub struct Move {
    src: Square,
    dest: Square,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.src, self.dest)
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.src != other.src {
            self.src.cmp(&other.src)
        } else if self.dest != other.dest {
            self.dest.cmp(&other.dest)
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Move {
    pub fn new(src: Square, dest: Square) -> Move {
        Move {
            src,
            dest,
        }
    }

    pub fn src(&self) -> Square {
        self.src
    }

    pub fn dest(&self) -> Square {
        self.dest
    }
}

impl MoveGen {
    pub fn with_mask(board: &Board, mask: Mask) -> MoveGen {
        let mut movelist = vec![];
        for src in board.pieces_to_move() {
            let moves = match mask {
                Mask::None => get_pawn_quiets(src, board.side_to_move(), board.occupied()) ^ get_pawn_attacks(src, board.side_to_move(), board.pieces_not_to_move()),
                Mask::Capture => get_pawn_attacks(src, board.side_to_move(), board.pieces_not_to_move()),
                Mask::Push => get_pawn_quiets(src, board.side_to_move(), board.occupied()),
            };
            if moves != EMPTY_BB {
                movelist.push(SquareAndBitBoard { sq: src, bb: moves })
            }
        }

        MoveGen {
            moves: movelist,
            index: 0,
        }
    }

    pub fn new(board: &Board) -> MoveGen {
        MoveGen::with_mask(board, Mask::None)

    }
}

impl ExactSizeIterator for MoveGen {
    fn len(&self) -> usize {
        let mut len = 0;
        for moves in &self.moves {
            if moves.bb == EMPTY_BB {
                break;
            }

            len += moves.bb.popcnt() as usize;
        }
        len
    }
}

impl Iterator for MoveGen {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let moves = &mut self.moves.get_mut(self.index)?;
        let dest = moves.bb.to_square();

        moves.bb ^= BitBoard::from_square(dest);
        if moves.bb == EMPTY_BB {
            self.index += 1;
        }
        Some(Move::new(moves.sq, dest))
    }
}
