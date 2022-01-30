use std::{collections::HashMap, hash::Hash};

use super::*;

use board_game::board::Board as BoardTrait;

// tweaked perft from board-game
pub fn perft<B: BoardTrait>(board: &B, depth: u32) -> u64 {
    fn perft_recurse<B: BoardTrait + Hash>(
        map: &mut HashMap<(B, u32), u64>,
        board: B,
        depth: u32,
    ) -> u64 {
        println!("{board}");
        if depth == 0 {
            return 1;
        }
        if board.is_done() {
            return 0;
        }
        if depth == 1 {
            return board
                .available_moves()
                .inspect(|mv| println!("{mv}"))
                .count() as u64;
        }

        // we need keys (B, depth) because otherwise we risk miscounting if the same board is encountered at different depths
        let key = (board, depth);
        let board = &key.0;

        if let Some(&p) = map.get(&key) {
            return p;
        }

        let mut p = 0;
        board.available_moves().for_each(|mv: B::Move| {
            let new_board = board.clone_and_play(mv);
            println!("{mv}");
            println!("{new_board}");
            p += perft_recurse(map, new_board, depth - 1);
        });

        map.insert(key, p);
        p
    }
    let mut map = HashMap::default();
    perft_recurse(&mut map, board.clone(), depth)
}

pub fn inspect_moves(board: &Board) {
    if board
        .available_moves()
        .inspect(|mv| println!("{mv}"))
        .count()
        == 0
    {
        println!("No moves");
    }
}

#[test]
fn board_perft() {
    let board_3x3 = Board::new(3);

    assert_eq!(perft(&board_3x3, 1), 3);
    assert_eq!(perft(&board_3x3, 2), 10);
    assert_eq!(perft(&board_3x3, 3), 28);
}

#[test]
fn play_move() {
    let mut board = Board::new(3);
    board.play(Move::new(Square::A1, Square::A2));
    println!("{board}");
    board.play(Move::new(Square::B3, Square::A2));
    println!("{board}");
}

#[test]
fn outcome() {
    let mut board_win_white = Board::new(3);
    board_win_white.play(Move::new(Square::A1, Square::A2));
    board_win_white.play(Move::new(Square::C3, Square::C2));
    board_win_white.play(Move::new(Square::A2, Square::B3));
    println!("White wins:\n{board_win_white}");
    assert_eq!(board_win_white.outcome(), Some(Outcome::WonBy(Player::A)));

    let mut board_win_black = Board::new(3);
    board_win_black.play(Move::new(Square::A1, Square::A2));
    board_win_black.play(Move::new(Square::B3, Square::A2));
    board_win_black.play(Move::new(Square::C1, Square::C2));
    board_win_black.play(Move::new(Square::A2, Square::B1));
    println!("Black wins:\n{board_win_black}");
    assert_eq!(board_win_black.outcome(), Some(Outcome::WonBy(Player::B)));

    let mut board_draw = Board::new(3);
    board_draw.play(Move::new(Square::A1, Square::A2));
    board_draw.play(Move::new(Square::B3, Square::B2));
    board_draw.play(Move::new(Square::C1, Square::C2));
    inspect_moves(&board_draw);
    println!("Draw:\n{board_draw}");
    assert_eq!(board_draw.outcome(), Some(Outcome::Draw));
}
