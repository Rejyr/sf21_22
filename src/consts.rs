//! Useful bitboard constants

use chess::BitBoard;

/// An empty bitboard
pub const EMPTY: u64 = 0;
/// An empty bitboard of [`chess`](chess)'s [`BitBoard`](chess::BitBoard) type
pub const EMPTY_BB: BitBoard = BitBoard(EMPTY);
/// A full bitboard
pub const UNIVERSAL: u64 = 0xFFFFFFFFFFFFFFFF;

/// The files of a bitboard, from file A \[0\] to file H \[7\]
pub const FILES: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

// ranks 1-8
/// The ranks of a bitboard, from rank 1 \[0\] to rank 8 \[7\]
pub const RANKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];

/// The playing area for hexapawn.
/// [`BOARD_MASKS`](BOARD_MASKS)`\[0\]` and [`BOARD_MASKS`](BOARD_MASKS)`[1]` are padding
pub const BOARD_MASKS: [u64; 8] = [
    0,
    0,
    0x0000000000070707,
    0x000000000F0F0F0F,
    0x0000001F1F1F1F1F,
    0x00003F3F3F3F3F3F,
    0x007F7F7F7F7F7F7F,
    0xFFFFFFFFFFFFFFFF,
];

/// The starting positions for white.
/// [`START_POS_WHITE`](START_POS_WHITE)`\[0\]` and [`START_POS_WHITE`](START_POS_WHITE)`\[1\]` are padding
pub const START_POS_WHITE: [u64; 8] = [
    0,
    0,
    0x0000000000000007,
    0x000000000000000F,
    0x000000000000001F,
    0x000000000000003F,
    0x000000000000007F,
    0x00000000000000FF,
];

/// The starting positions for black.
/// [`START_POS_BLACK`](START_POS_BLACK)`\[0\]` and [`START_POS_BLACK`](START_POS_BLACK)`\[1\]` are padding
pub const START_POS_BLACK: [u64; 8] = [
    0,
    0,
    0x0000000000070000,
    0x000000000F000000,
    0x0000001F00000000,
    0x00003F0000000000,
    0x007F000000000000,
    0xFF00000000000000,
];
