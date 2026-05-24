use std::fmt;

#[derive(Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct PieceOnBoard {
    piece: Piece,
    color: PieceColor,
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
