use anyhow::{Context, Result, bail};

use crate::game::bitboard::Bitboard;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub const ALL: [PieceColor; 2] = [PieceColor::White, PieceColor::Black];
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub const ALL: [PieceKind; 6] = [
        PieceKind::Pawn,
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Rook,
        PieceKind::Queen,
        PieceKind::King,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub rank: u8,
    pub file: u8,
}

impl Square {
    pub fn from_bit_index(index: u8) -> Self {
        let rank = index / 8;
        let file = index % 8;

        Self { rank, file }
    }

    pub fn as_bit_index(&self) -> u8 {
        self.rank * 8 + self.file
    }

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
            file: "abcdefgh"
                .chars()
                .position(|x| x == letter)
                .context("Invalid letter in notation")? as u8,
            rank: number as u8 - 1,
        })
    }

    pub fn as_algebraic_notation(&self) -> Result<String> {
        let letter = "abcdefgh"
            .chars()
            .nth(self.file as usize)
            .context("Invalid square column")?;
        let number = self.rank + 1;

        if number > 8 {
            bail!("Invalid square row");
        }

        Ok(format!("{}{}", letter, number))
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PieceBitboards([Bitboard; 6]);

impl std::ops::Index<PieceKind> for PieceBitboards {
    type Output = Bitboard;

    fn index(&self, kind: PieceKind) -> &Self::Output {
        &self.0[kind as usize]
    }
}

impl std::ops::IndexMut<PieceKind> for PieceBitboards {
    fn index_mut(&mut self, kind: PieceKind) -> &mut Self::Output {
        &mut self.0[kind as usize]
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ColorBitboards([PieceBitboards; 2]);

impl std::ops::Index<PieceColor> for ColorBitboards {
    type Output = PieceBitboards;

    fn index(&self, color: PieceColor) -> &Self::Output {
        &self.0[color as usize]
    }
}

impl std::ops::IndexMut<PieceColor> for ColorBitboards {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        &mut self.0[color as usize]
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub bb_pieces: ColorBitboards,
}

impl Position {
    pub fn bb_color(&self, color: PieceColor) -> Bitboard {
        PieceKind::ALL.iter().fold(Bitboard::EMPTY, |acc, &kind| {
            acc | self.bb_pieces[color][kind]
        })
    }

    pub fn bb_all(&self) -> Bitboard {
        self.bb_color(PieceColor::White) | self.bb_color(PieceColor::Black)
    }
}

#[derive(Debug)]
pub struct GameState {
    pub position: Position,
    pub color_to_move: PieceColor,
    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,
    pub half_turn_count: u8,
}

impl GameState {
    pub fn initial_position() -> Self {
        let mut position = Position::default();

        position.bb_pieces[PieceColor::White][PieceKind::Pawn] = Bitboard(0xFF00);
        position.bb_pieces[PieceColor::White][PieceKind::Knight] = Bitboard(0x42);
        position.bb_pieces[PieceColor::White][PieceKind::Bishop] = Bitboard(0x24);
        position.bb_pieces[PieceColor::White][PieceKind::Rook] = Bitboard(0x81);
        position.bb_pieces[PieceColor::White][PieceKind::Queen] = Bitboard(0x8);
        position.bb_pieces[PieceColor::White][PieceKind::King] = Bitboard(0x10);
        position.bb_pieces[PieceColor::Black][PieceKind::Pawn] = Bitboard(0xFF000000000000);
        position.bb_pieces[PieceColor::Black][PieceKind::Knight] = Bitboard(0x4200000000000000);
        position.bb_pieces[PieceColor::Black][PieceKind::Bishop] = Bitboard(0x2400000000000000);
        position.bb_pieces[PieceColor::Black][PieceKind::Rook] = Bitboard(0x8100000000000000);
        position.bb_pieces[PieceColor::Black][PieceKind::Queen] = Bitboard(0x800000000000000);
        position.bb_pieces[PieceColor::Black][PieceKind::King] = Bitboard(0x1000000000000000);

        Self {
            position,
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
        if !self.position.bb_all().contains(&square) {
            return None;
        }

        if self.position.bb_color(PieceColor::White).contains(&square) {
            for kind in PieceKind::ALL {
                if self.position.bb_pieces[PieceColor::White][kind].contains(&square) {
                    return Some(Piece {
                        color: PieceColor::White,
                        piece: kind,
                    });
                }
            }
        }

        for kind in PieceKind::ALL {
            if self.position.bb_pieces[PieceColor::Black][kind].contains(&square) {
                return Some(Piece {
                    color: PieceColor::Black,
                    piece: kind,
                });
            }
        }

        return None;
    }

    pub fn replace_piece(&mut self, square: &Square, new_piece: Option<Piece>) {
        for kind in PieceKind::ALL {
            self.position.bb_pieces[PieceColor::White][kind].clear(square);
            self.position.bb_pieces[PieceColor::Black][kind].clear(square);
        }

        if let Some(piece) = new_piece {
            self.position.bb_pieces[piece.color][piece.piece].set(square);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_as_bit_index() {
        assert_eq!(Square { rank: 0, file: 0 }.as_bit_index(), 0);

        assert_eq!(Square { rank: 1, file: 3 }.as_bit_index(), 11);
    }

    #[test]
    fn test_square_from_bit_index() {
        assert_eq!(Square::from_bit_index(0), Square { rank: 0, file: 0 });

        assert_eq!(Square::from_bit_index(11), Square { rank: 1, file: 3 });
    }

    #[test]
    fn test_square_as_algebraic_notation() {
        assert_eq!(
            Square { rank: 0, file: 0 }.as_algebraic_notation().unwrap(),
            "a1"
        );

        assert_eq!(
            Square { rank: 1, file: 3 }.as_algebraic_notation().unwrap(),
            "d2"
        );
    }

    #[test]
    fn test_square_from_algebraic_notation() {
        assert_eq!(
            Square::from_algebraic_notation("a1").unwrap(),
            Square { rank: 0, file: 0 }
        );

        assert_eq!(
            Square::from_algebraic_notation("d2").unwrap(),
            Square { rank: 1, file: 3 }
        );
    }
}
