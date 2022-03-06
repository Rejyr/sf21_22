//! Move generation for hexapawn

use std::{cmp::Ordering, fmt::Display};

use chess::{get_pawn_attacks, get_pawn_quiets, BitBoard, Square};

use crate::{board::Board, consts::EMPTY_BB};

/// A struct containing a square (the position of a pawn) and a bitboard (possible moves)
pub struct SquareAndBitBoard {
    sq: Square,
    bb: BitBoard,
}

/// Move generation masks
pub enum Mask {
    /// All moves
    None,
    /// Only capture moves
    Capture,
    /// Only push moves
    Push,
}

/// Incremental move generation through Iterator
pub struct MoveGen {
    /// All pieces to move and their possible moves
    moves: Vec<SquareAndBitBoard>,
    /// The current SquareAndBitBoard used for move gen
    index: usize,
}

/// A move
#[derive(Clone, Copy, Eq, PartialEq, Default, Debug, Hash)]
pub struct Move {
    /// From which square
    src: Square,
    /// To which square
    dest: Square,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.src, self.dest)
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // if sources aren't equal, compare them
        if self.src != other.src {
            self.src.cmp(&other.src)
        // if destinations aren't equal, compare them
        } else if self.dest != other.dest {
            self.dest.cmp(&other.dest)
        // otherwise, they're equal
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
    /// Creates a new `Move` from a source and destination
    pub fn new(src: Square, dest: Square) -> Move {
        Move { src, dest }
    }

    /// Returns the source of the `Move`
    pub fn src(&self) -> Square {
        self.src
    }

    /// Returns the destination of the `Move`
    pub fn dest(&self) -> Square {
        self.dest
    }
}

impl MoveGen {
    /// Creates a new `MoveGen` with a move generation mask
    pub fn with_mask(board: &Board, mask: Mask) -> MoveGen {
        let mut movelist = vec![];
        // for every piece to move
        for src in board.pieces_to_move() {
            let moves = match mask {
                // pawn captures and pushes
                Mask::None => {
                    get_pawn_quiets(src, board.side_to_move(), board.occupied())
                        ^ get_pawn_attacks(src, board.side_to_move(), board.pieces_not_to_move())
                }
                // only pawn captures
                Mask::Capture => {
                    get_pawn_attacks(src, board.side_to_move(), board.pieces_not_to_move())
                }
                // only pawn pushes
                Mask::Push => get_pawn_quiets(src, board.side_to_move(), board.occupied()),
            };
            // if there are moves, add it
            if moves != EMPTY_BB {
                movelist.push(SquareAndBitBoard { sq: src, bb: moves })
            }
        }

        MoveGen {
            moves: movelist,
            index: 0,
        }
    }

    /// Creates a new `MoveGen` with no mask
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
        // Get the currently used SquareAndBitBoard. if there's none,finish
        let moves = &mut self.moves.get_mut(self.index)?;
        // Get the least signifigant ones bit
        let dest = moves.bb.to_square();

        // remove that bit from the SquareAndBitBoard
        moves.bb ^= BitBoard::from_square(dest);
        // if the SquareAndBitBoard is out of moves, increment the index
        if moves.bb == EMPTY_BB {
            self.index += 1;
        }
        // create a move from the square to move and a possible move
        Some(Move::new(moves.sq, dest))
    }
}
