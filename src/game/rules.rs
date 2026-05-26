use std::cmp::{Ordering, max};

use anyhow::{Result, bail};
use macroquad::prelude::scene::Handle;

use crate::game::{
    bitboard::Bitboard,
    game_state::{GameState, Piece, PieceColor, PieceKind, Square},
};

pub fn can_touch_piece(state: &GameState, square: &Square) -> bool {
    // If there's no piece on the square, can't move it
    let Some(piece) = state.get_piece_at_square(square) else {
        return false;
    };

    // If the piece does not have the color that's at turn, can't move it
    if piece.color != state.color_to_move {
        return false;
    }

    true
}

pub fn attacks_for_knight(state: &GameState, square: &Square) -> Bitboard {
    let mut src = Bitboard::EMPTY;
    src.set(square);

    ((src & !Bitboard::FILE_H)  << 17)  // +2r +1f
        | ((src & !Bitboard::FILE_A)  << 15)  // +2r -1f
        | ((src & !(Bitboard::FILE_G | Bitboard::FILE_H)) << 10)  // +1r +2f
        | ((src & !(Bitboard::FILE_A | Bitboard::FILE_B)) <<  6)  // +1r -2f
        | ((src & !(Bitboard::FILE_G | Bitboard::FILE_H)) >>  6)  // -1r +2f
        | ((src & !(Bitboard::FILE_A | Bitboard::FILE_B)) >> 10)  // -1r -2f
        | ((src & !Bitboard::FILE_H)  >> 15)  // -2r +1f
        | ((src & !Bitboard::FILE_A)  >> 17) // -2r -1f
}

pub fn attacks_for(state: &GameState, piece: &Piece, square: &Square) -> Bitboard {
    let mut attacks = match piece.piece {
        PieceKind::Knight => attacks_for_knight(state, square),
        _ => Bitboard::EMPTY, // TODO: implement
    };

    // Remove attacks that would touch friendly pieces
    attacks &= !state.position.bb_color(state.color_to_move);

    attacks
}
