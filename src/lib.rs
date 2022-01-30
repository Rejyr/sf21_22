use board_game::board::Outcome;
use chess::get_pawn_attacks;
use chess::Square;
use chess::get_pawn_quiets;
use consts::EMPTY_BB;
use internal_iterator::Internal;
use internal_iterator::InternalIterator;
use internal_iterator::IteratorExt;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::ControlFlow;

use board_game::board::Board as BoardTrait;
use board_game::board::BoardMoves;
use board_game::board::Player;
use board_game::board::UnitSymmetryBoard;
use chess::BitBoard;
use chess::Color;
use consts::RANKS;

use crate::consts::START_POS_BLACK;
use crate::consts::START_POS_WHITE;

// TODO: perft and tests
// TODO: Display implementation

#[cfg(test)]
mod tests;
pub mod consts;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    white: BitBoard,
    black: BitBoard,
    side_to_move: Color,
    size: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut white = self.white.0;
        let mut black = self.black.0;

        // get 8th rank, shift up
        for i in 0..8 {
            // shrink into u8
            let mut w_rank = ((white & RANKS[7]) >> 56) as u8;
            let mut b_rank = ((black & RANKS[7]) >> 56) as u8;

            for _ in 0..8 {
                // get first
                let w_sq = w_rank & 1;
                let b_sq = b_rank & 1;

                match (w_sq, b_sq) {
                    (0, 0) => f.write_str(" ")?,
                    (1, 0) => f.write_str("♙")?, // white pawn
                    (0, 1) => f.write_str("♟︎")?, // black pawn
                    (1, 1) => panic!("Board has two pawns in the same place"),
                    _ => {}
                }

                w_rank >>= 1;
                b_rank >>= 1;
            }

            f.write_fmt(format_args!("|{}\n", 8 - i))?;
            // shift board up 1
            white <<= 8;
            black <<= 8;
        }

        f.write_str("--------*\n")?;
        f.write_str("abcdefgh\n")?;
        f.write_str("01234567\n")?;

        Ok(())
    }
}

impl Board {
    pub fn new(size: usize) -> Self {
        assert!(size > 2 && size < 9, "Invalid size, must be 3 to 8");
        Board {
            white: BitBoard(START_POS_WHITE[size - 1]),
            black: BitBoard(START_POS_BLACK[size - 1]),
            side_to_move: Color::White,
            size,
        }
    }

    pub fn pieces(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    pub fn pieces_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }

    pub fn pieces_to_move(&self) -> BitBoard {
        self.pieces(self.side_to_move)
    }

    pub fn pieces_to_move_mut(&mut self) -> &mut BitBoard {
        self.pieces_mut(self.side_to_move)
    }

    pub fn pieces_not_to_move(&self) -> BitBoard {
        self.pieces(!self.side_to_move)
    }

    pub fn pieces_not_to_move_mut(&mut self) -> &mut BitBoard {
        self.pieces_mut(!self.side_to_move)
    }

    pub fn empty(&self) -> BitBoard {
        !self.occupied()
    }

    pub fn occupied(&self) -> BitBoard {
        self.white | self.black
    }
}

impl UnitSymmetryBoard for Board {}

impl BoardTrait for Board {
    type Move = Move;

    fn next_player(&self) -> board_game::board::Player {
        match self.side_to_move {
            Color::White => Player::A,
            Color::Black => Player::B,
        }
    }

    fn is_available_move(&self, mv: Self::Move) -> bool {
        MoveGen::new(self).any(|x| x == mv)
    }

    fn play(&mut self, mv: Self::Move) {
        let src_bb = BitBoard::from_square(mv.src);
        let dest_bb = BitBoard::from_square(mv.dest);
        *self.pieces_to_move_mut() ^= src_bb | dest_bb;
        *self.pieces_not_to_move_mut() &= !dest_bb;
        self.side_to_move = !self.side_to_move;
    }

    fn outcome(&self) -> Option<board_game::board::Outcome> {
        if self.white & BitBoard(RANKS[self.size - 1]) != EMPTY_BB {
            Some(Outcome::WonBy(Player::A))
        } else if self.black & BitBoard(RANKS[0]) != EMPTY_BB {
            Some(Outcome::WonBy(Player::B))
        } else if MoveGen::new(self).len() == 0 {
            Some(Outcome::Draw)
        } else {
            None
        }
    }

    fn can_lose_after_move() -> bool {
        false
    }
}

impl<'a> BoardMoves<'a, Board> for Board {
    type AllMovesIterator = AllMoves;

    type AvailableMovesIterator = Internal<MoveGen>;

    fn all_possible_moves() -> Self::AllMovesIterator {
        AllMoves
    }

    fn available_moves(&'a self) -> Self::AvailableMovesIterator {
        MoveGen::new(self).into_internal()
    }
}

pub struct AllMoves;

impl InternalIterator for AllMoves {
    type Item = Move;

    fn try_for_each<R, F>(self, mut f: F) -> std::ops::ControlFlow<R>
    where
        F: FnMut(Self::Item) -> std::ops::ControlFlow<R>,
    {
        for from in chess::ALL_SQUARES {
            for to in chess::ALL_SQUARES {
                f(Move::new(from, to))?;
            }
        }

        ControlFlow::Continue(())
    }
}

pub struct SquareAndBitBoard {
    sq: Square,
    bb: BitBoard,
}

pub struct MoveGen {
    moves: Vec<SquareAndBitBoard>,
    index: usize,
}

// trimmed ChessMove from chess
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Default, Debug, Hash)]
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
            return self.src.cmp(&other.src)
        } if self.dest != other.dest {
            self.dest.cmp(&other.dest)
        } else {
            Ordering::Equal
        }
    }
}

impl Move {
    pub fn new(src: Square, dest: Square) -> Move {
        Move {
            src,
            dest,
        }
    }
}

impl MoveGen {
    pub fn new(board: &Board) -> MoveGen {
        let mut movelist = vec![];
        for src in board.pieces_to_move() {
            let moves = get_pawn_quiets(src, board.side_to_move, board.occupied()) ^ get_pawn_attacks(src, board.side_to_move, board.pieces_not_to_move());
            if moves != EMPTY_BB {
                movelist.push(SquareAndBitBoard { sq: src, bb: moves })
            }
        }

        MoveGen {
            moves: movelist,
            index: 0,
        }
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
