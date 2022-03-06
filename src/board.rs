//! Board representation of hexapawn

use std::fmt::Display;
use std::ops::ControlFlow;

use board_game::board::Board as BoardTrait;
use board_game::board::BoardMoves;
use board_game::board::Outcome;
use board_game::board::Player;
use board_game::board::UnitSymmetryBoard;
use chess::{BitBoard, Color};
use internal_iterator::Internal;
use internal_iterator::InternalIterator;
use internal_iterator::IteratorExt;

use crate::consts::EMPTY_BB;
use crate::consts::{RANKS, START_POS_BLACK, START_POS_WHITE};
use crate::move_gen::Move;
use crate::move_gen::MoveGen;

/// A representation of hexapawn of various sizes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    /// White's pawns
    white: BitBoard, // stored as a unsigned 64 bit integer bitboard
    /// Black's pawns
    black: BitBoard,
    /// The side to move
    side_to_move: Color,
    /// The size of the board, from 3 to 8
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
                    (1, 0) => f.write_str("♙")?,    // white pawn
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
    /// Creates a new `Board`
    /// The size must be 3 to 8
    pub fn new(size: usize) -> Self {
        assert!(size > 2 && size < 9, "Invalid size, must be 3 to 8");
        Board {
            white: BitBoard(START_POS_WHITE[size - 1]),
            black: BitBoard(START_POS_BLACK[size - 1]),
            side_to_move: Color::White,
            size,
        }
    }

    /// Returns the `BitBoard` of a certain color's bitboard
    pub fn pieces(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    /// Returns a mutable reference to a certain color's `BitBoard`
    pub fn pieces_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }

    /// Returns the `BitBoard` of the pieces to move
    pub fn pieces_to_move(&self) -> BitBoard {
        self.pieces(self.side_to_move)
    }

    /// Returns a mutable reference to the `BitBoard` of the pieces to move
    pub fn pieces_to_move_mut(&mut self) -> &mut BitBoard {
        self.pieces_mut(self.side_to_move)
    }

    /// Returns the `BitBoard` of the pieces not to move
    pub fn pieces_not_to_move(&self) -> BitBoard {
        self.pieces(!self.side_to_move)
    }

    /// Returns a mutable reference to the `BitBoard` of the pieces not to move
    pub fn pieces_not_to_move_mut(&mut self) -> &mut BitBoard {
        self.pieces_mut(!self.side_to_move)
    }

    /// Returns the side to move, White or Black
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Returns a `BitBoard` of all empty squares
    pub fn empty(&self) -> BitBoard {
        !self.occupied()
    }

    /// Returns a `BitBoard` of all occupied squares
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
        // Convert the squares to bitboards
        let src_bb = BitBoard::from_square(mv.src());
        let dest_bb = BitBoard::from_square(mv.dest());
        *self.pieces_to_move_mut() ^= src_bb | dest_bb; // Move the pawn
        *self.pieces_not_to_move_mut() &= !dest_bb; // Remove the captured pawn
        self.side_to_move = !self.side_to_move; // Switch sides to move
    }

    fn outcome(&self) -> Option<board_game::board::Outcome> {
        // if white's pawns reaches black's starting rank, white wins
        if self.white & BitBoard(RANKS[self.size - 1]) != EMPTY_BB {
            Some(Outcome::WonBy(Player::A))
        // if white's pawns reaches black's starting rank, white wins
        } else if self.black & BitBoard(RANKS[0]) != EMPTY_BB {
            Some(Outcome::WonBy(Player::B))
        // if there are no moves, it's a draw
        } else if MoveGen::new(self).len() == 0 {
            Some(Outcome::Draw)
        // otherwise, the game is still ongoing
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

#[doc(hidden)]
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
