use board_game::board::Outcome;
use chess::get_pawn_moves;
use chess::Square;
use consts::EMPTY_BB;
use internal_iterator::Internal;
use internal_iterator::InternalIterator;
use internal_iterator::IteratorExt;
use std::fmt::Display;
use std::ops::ControlFlow;

use board_game::board::Board as BoardTrait;
use board_game::board::BoardMoves;
use board_game::board::Player;
use board_game::board::UnitSymmetryBoard;
use chess::BitBoard;
use chess::ChessMove;
use chess::Color;
use consts::RANKS;

use crate::consts::EMPTY;
use crate::consts::START_POS_BLACK;
use crate::consts::START_POS_WHITE;

// TODO: perft and tests
// TODO: Display implementation

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
        todo!()
    }
}

impl Board {
    pub fn new(size: usize) -> Self {
        assert!(size > 2 && size < 9, "Invalid size, must be 3 to 8");
        Board {
            white: BitBoard(START_POS_WHITE[size]),
            black: BitBoard(START_POS_BLACK[size]),
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

    pub fn empty(&self) -> BitBoard {
        !self.occupied()
    }

    pub fn occupied(&self) -> BitBoard {
        self.white | self.black
    }
}

impl UnitSymmetryBoard for Board {}

impl BoardTrait for Board {
    type Move = ChessMove;

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
        *self.pieces_to_move_mut() = BitBoard::from_square(mv.get_source()) | BitBoard::from_square(mv.get_dest());
        self.side_to_move = !self.side_to_move;
    }

    fn outcome(&self) -> Option<board_game::board::Outcome> {
        if self.white & BitBoard(RANKS[self.size]) != EMPTY_BB {
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
    type Item = ChessMove;

    fn try_for_each<R, F>(self, mut f: F) -> std::ops::ControlFlow<R>
    where
        F: FnMut(Self::Item) -> std::ops::ControlFlow<R>,
    {
        for from in chess::ALL_SQUARES {
            for to in chess::ALL_SQUARES {
                f(ChessMove::new(from, to, None))?;

                for piece in chess::PROMOTION_PIECES {
                    f(ChessMove::new(from, to, Some(piece)))?;
                }
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

impl MoveGen {
    pub fn new(board: &Board) -> MoveGen {
        let mut movelist = vec![];
        for src in board.pieces_to_move() {
            let moves = get_pawn_moves(src, board.side_to_move, board.occupied());
            if moves != BitBoard(0) {
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
            if moves.bb == BitBoard(0) {
                break;
            }

            len += moves.bb.popcnt() as usize;
        }
        len
    }
}

impl Iterator for MoveGen {
    type Item = ChessMove;

    fn next(&mut self) -> Option<Self::Item> {
        let moves = &mut self.moves.get_mut(self.index)?;
        let dest = moves.bb.to_square();

        moves.bb ^= BitBoard::from_square(dest);
        if moves.bb == BitBoard(0) {
            self.index += 1;
        }
        Some(ChessMove::new(moves.sq, dest, None))
    }
}
