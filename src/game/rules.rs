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

pub fn attacks_for_knight(state: &GameState, _: &Piece, square: &Square) -> Bitboard {
    let mut src = Bitboard::EMPTY;
    src.set(square);

    ((src & !Bitboard::FILE_H) << 17)  // +2r +1f
        | ((src & !Bitboard::FILE_A) << 15)  // +2r -1f
        | ((src & !(Bitboard::FILE_G | Bitboard::FILE_H)) << 10)  // +1r +2f
        | ((src & !(Bitboard::FILE_A | Bitboard::FILE_B)) <<  6)  // +1r -2f
        | ((src & !(Bitboard::FILE_G | Bitboard::FILE_H)) >>  6)  // -1r +2f
        | ((src & !(Bitboard::FILE_A | Bitboard::FILE_B)) >> 10)  // -1r -2f
        | ((src & !Bitboard::FILE_H) >> 15)  // -2r +1f
        | ((src & !Bitboard::FILE_A) >> 17) // -2r -1f
}

pub fn attacks_for_king(state: &GameState, _: &Piece, square: &Square) -> Bitboard {
    let mut src = Bitboard::EMPTY;
    src.set(square);

    ((src & !Bitboard::FILE_H) << 1)  // +1f
        | ((src & !Bitboard::FILE_H) >> 7) // -1r +1f
        | (src >> 8) // -1r
        | ((src & !Bitboard::FILE_A) >> 9) // -1r -1f
        | ((src & !Bitboard::FILE_A) >> 1) // -1f
        | ((src & !Bitboard::FILE_A) << 7) // +1r -1f
        | (src << 8) // +1r
        | ((src & !Bitboard::FILE_H) << 9) // +1r +1f
}

pub fn attacks_for_pawn(state: &GameState, piece: &Piece, square: &Square) -> Bitboard {
    let mut src = Bitboard::EMPTY;
    let enemy_bb = state.position.bb_color(!piece.color);
    src.set(square);

    let single_pushes = (if piece.color == PieceColor::White {
        src << 8
    } else {
        src >> 8
    }) & !enemy_bb;

    let double_pushes = (if piece.color == PieceColor::White {
        single_pushes << 8
    } else {
        single_pushes >> 8
    }) & !enemy_bb;

    let captures = (if piece.color == PieceColor::White {
        (src & !Bitboard::FILE_A) << 7 | (src & !Bitboard::FILE_H) << 9
    } else {
        (src & !Bitboard::FILE_H) >> 7 | (src & !Bitboard::FILE_A) >> 9
    }) & enemy_bb;

    single_pushes | double_pushes | captures
}

pub fn attacks_for(state: &GameState, piece: &Piece, square: &Square) -> Bitboard {
    let mut attacks = match piece.piece {
        PieceKind::Pawn => attacks_for_pawn(state, piece, square),
        PieceKind::Knight => attacks_for_knight(state, piece, square),
        PieceKind::King => attacks_for_king(state, piece, square),
        _ => Bitboard::FULL, // TODO: implement
    };

    // Remove attacks that would touch friendly pieces
    attacks &= !state.position.bb_color(state.color_to_move);

    // TODO: non-pseudo-legal moves (check, pins, etc.)

    attacks
}
