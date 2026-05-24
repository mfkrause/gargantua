use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece: PieceKind,
    pub color: PieceColor,
}

#[derive(Debug, Clone, Copy)]
pub struct CastlingRights {
    king_side: bool,
    queen_side: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub column: u8,
    pub row: u8,
}

impl Square {
    pub fn from_algebraic_notation(notation: &str) -> Result<Self> {
        let lowercased_notation = notation.to_lowercase();
        let mut chars = lowercased_notation.chars();
        let letter = chars.nth(0).context("Input string is too short")?;
        let number = chars
            .nth(0)
            .context("Input string is too short")?
            .to_digit(10)
            .context("Couldn't parse number")?;

        if number > 8 {
            bail!("Invalid number in notation")
        }

        Ok(Square {
            column: "abcdefgh"
                .chars()
                .position(|x| x == letter)
                .context("Invalid letter in notation")? as u8,
            row: number as u8 - 1,
        })
    }

    pub fn as_algebraic_notation(&self) -> Result<String> {
        let letter = "abcdefgh"
            .chars()
            .nth(self.column as usize)
            .context("Invalid square column")?;
        let number = self.row + 1;

        if number > 8 {
            bail!("Invalid square row");
        }

        Ok(format!("{}{}", letter, number))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub board: [[Option<Piece>; 8]; 8],
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
                    Some(Piece {
                        piece: PieceKind::Rook,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Knight,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Bishop,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Queen,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::King,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Bishop,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Knight,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Rook,
                        color: PieceColor::White,
                    }),
                ],
                [
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::White,
                    }),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Pawn,
                        color: PieceColor::Black,
                    }),
                ],
                [
                    Some(Piece {
                        piece: PieceKind::Rook,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Knight,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Bishop,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Queen,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::King,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Bishop,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Knight,
                        color: PieceColor::Black,
                    }),
                    Some(Piece {
                        piece: PieceKind::Rook,
                        color: PieceColor::Black,
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

    pub fn get_piece_at_square(&self, square: &Square) -> Option<Piece> {
        self.board[square.column as usize][square.row as usize]
    }

    pub fn replace_piece(&mut self, square: &Square, new_piece: Option<Piece>) {
        self.board[square.column as usize][square.row as usize] = new_piece;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_algebraic_notation() {
        assert_eq!(
            Square::from_algebraic_notation("a1").unwrap(),
            Square { column: 0, row: 0 }
        );

        assert_eq!(
            Square::from_algebraic_notation("h8").unwrap(),
            Square { column: 7, row: 7 }
        );
    }

    #[test]
    fn test_square_to_algebraic_notation() {
        assert_eq!(
            Square { column: 0, row: 0 }
                .as_algebraic_notation()
                .unwrap(),
            "a1"
        );

        assert_eq!(
            Square { column: 7, row: 7 }
                .as_algebraic_notation()
                .unwrap(),
            "h8"
        );
    }
}
