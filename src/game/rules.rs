use std::cmp::{Ordering, max};

use anyhow::{Result, bail};

use crate::game::game_state::{GameState, Piece, PieceColor, PieceKind, Square};

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

#[derive(Debug, PartialEq)]
pub enum SlidingMoveDirection {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

pub struct SlidingMoveDistance {
    direction: SlidingMoveDirection,
    distance: u8,
}

pub fn get_distance_of_sliding_move(
    source_square: &Square,
    target_square: &Square,
) -> Result<SlidingMoveDistance> {
    let horizontal = target_square.file as i8 - source_square.file as i8;
    let vertical = target_square.rank as i8 - source_square.rank as i8;

    if horizontal != 0 && vertical != 0 && horizontal.abs() != vertical.abs() {
        bail!("Invalid move direction");
    }

    let distance = max(horizontal, vertical) as u8;

    let direction = match horizontal.abs().cmp(&vertical.abs()) {
        // Vertical
        Ordering::Less => {
            if vertical > 0 {
                SlidingMoveDirection::North
            } else {
                SlidingMoveDirection::South
            }
        }
        // Horizontal
        Ordering::Greater => {
            if horizontal > 0 {
                SlidingMoveDirection::East
            } else {
                SlidingMoveDirection::West
            }
        }
        // Diagonal
        Ordering::Equal => {
            if horizontal > 0 && vertical > 0 {
                SlidingMoveDirection::NorthEast
            } else if horizontal > 0 && vertical < 0 {
                SlidingMoveDirection::SouthEast
            } else if horizontal < 0 && vertical < 0 {
                SlidingMoveDirection::SouthWest
            } else {
                SlidingMoveDirection::NorthWest
            }
        }
    };

    Ok(SlidingMoveDistance {
        direction,
        distance,
    })
}

fn can_move_pawn(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
    distance: &SlidingMoveDistance,
) -> Result<bool> {
    if distance.distance > 2 {
        return Ok(false);
    }

    if !((distance.direction == SlidingMoveDirection::North
        && state.color_to_move == PieceColor::White)
        || (distance.direction == SlidingMoveDirection::South
            && state.color_to_move == PieceColor::Black))
    {
        return Ok(false);
    }

    Ok(true)
}

fn can_move_bishop(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
    distance: &SlidingMoveDistance,
) -> Result<bool> {
    if let SlidingMoveDirection::North
    | SlidingMoveDirection::East
    | SlidingMoveDirection::South
    | SlidingMoveDirection::West = distance.direction
    {
        return Ok(false);
    }

    Ok(true)
}

fn can_move_knight(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
) -> Result<bool> {
    let horizontal = target_square.file as i8 - source_square.file as i8;
    let vertical = target_square.rank as i8 - source_square.rank as i8;

    if !((horizontal.abs() == 2 && vertical.abs() == 1)
        || (horizontal.abs() == 1 && vertical.abs() == 2))
    {
        return Ok(false);
    }

    Ok(true)
}

fn can_move_rook(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
    distance: &SlidingMoveDistance,
) -> Result<bool> {
    if let SlidingMoveDirection::NorthEast
    | SlidingMoveDirection::SouthEast
    | SlidingMoveDirection::SouthWest
    | SlidingMoveDirection::NorthWest = distance.direction
    {
        return Ok(false);
    }

    Ok(true)
}

fn can_move_queen(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
    distance: &SlidingMoveDistance,
) -> Result<bool> {
    Ok(true)
}

fn can_move_king(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
    distance: &SlidingMoveDistance,
) -> Result<bool> {
    if distance.distance > 1 {
        return Ok(false);
    }

    Ok(true)
}

pub fn can_move_piece(
    state: &GameState,
    source_square: &Square,
    target_square: &Square,
) -> Result<bool> {
    // If there's no piece on the square, can't move it
    let Some(piece) = state.get_piece_at_square(source_square) else {
        return Ok(false);
    };

    // If the piece does not have the color that's at turn, can't move it
    if piece.color != state.color_to_move {
        return Ok(false);
    }

    // If there's another piece with the same color on the target square, can't move there
    if let Some(piece_at_target_square) = state.get_piece_at_square(target_square)
        && piece_at_target_square.color == state.color_to_move
    {
        return Ok(false);
    }

    // The knight can jump across pieces
    if let PieceKind::Knight = piece.piece {
        return can_move_knight(state, source_square, target_square);
    }

    // Sliding pieces
    let distance = get_distance_of_sliding_move(source_square, target_square)?;

    match piece.piece {
        PieceKind::Pawn => can_move_pawn(state, source_square, target_square, &distance),
        PieceKind::Bishop => can_move_bishop(state, source_square, target_square, &distance),
        PieceKind::Rook => can_move_rook(state, source_square, target_square, &distance),
        PieceKind::Queen => can_move_queen(state, source_square, target_square, &distance),
        PieceKind::King => can_move_king(state, source_square, target_square, &distance),
        _ => Ok(false),
    }
}
