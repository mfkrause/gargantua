use std::fmt;

use macroquad::{
    color::WHITE,
    math::vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct PieceOnBoard {
    piece: Piece,
    color: PieceColor,
}

impl PieceOnBoard {
    pub async fn draw(self, x: f32, y: f32, w: f32, h: f32) {
        let texture: Texture2D = match self {
            PieceOnBoard {
                piece: Piece::Pawn,
                color: PieceColor::White,
            } => load_texture("assets/wP.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Rook,
                color: PieceColor::White,
            } => load_texture("assets/wR.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Knight,
                color: PieceColor::White,
            } => load_texture("assets/wN.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Bishop,
                color: PieceColor::White,
            } => load_texture("assets/wB.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Queen,
                color: PieceColor::White,
            } => load_texture("assets/wQ.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::King,
                color: PieceColor::White,
            } => load_texture("assets/wK.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Pawn,
                color: PieceColor::Black,
            } => load_texture("assets/bP.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Rook,
                color: PieceColor::Black,
            } => load_texture("assets/bR.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Knight,
                color: PieceColor::Black,
            } => load_texture("assets/bN.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Bishop,
                color: PieceColor::Black,
            } => load_texture("assets/bB.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::Queen,
                color: PieceColor::Black,
            } => load_texture("assets/bQ.png").await.unwrap(),
            PieceOnBoard {
                piece: Piece::King,
                color: PieceColor::Black,
            } => load_texture("assets/bK.png").await.unwrap(),
        };

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
    }
}

#[derive(Debug)]
pub struct CastlingRights {
    king_side: bool,
    queen_side: bool,
}

#[derive(Debug)]
pub struct GameState {
    pub board: [[Option<PieceOnBoard>; 8]; 8],
    pub color_to_move: PieceColor,
    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,
    pub half_turn_count: u8,
}

impl GameState {
    pub fn initial_position() -> Self {
        Self {
            board: [
                [
                    Some(PieceOnBoard {
                        piece: Piece::Rook,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Knight,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Bishop,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Queen,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::King,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Bishop,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Knight,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Rook,
                        color: PieceColor::Black,
                    }),
                ],
                [
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::Black,
                    }),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Pawn,
                        color: PieceColor::White,
                    }),
                ],
                [
                    Some(PieceOnBoard {
                        piece: Piece::Rook,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Knight,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Bishop,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Queen,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::King,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Bishop,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Knight,
                        color: PieceColor::White,
                    }),
                    Some(PieceOnBoard {
                        piece: Piece::Rook,
                        color: PieceColor::White,
                    }),
                ],
            ],
            color_to_move: PieceColor::White,
            white_castling_rights: CastlingRights {
                king_side: true,
                queen_side: true,
            },
            black_castling_rights: CastlingRights {
                king_side: true,
                queen_side: true,
            },
            half_turn_count: 0,
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Print visual representation of board
        write!(f, "{:?}", self)
    }
}
