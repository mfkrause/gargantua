use std::fmt::Display;

use anyhow::Result;

use crate::game::game_state::Square;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(!0);
    pub const FILE_A: Self = Self(0x0101010101010101);
    pub const FILE_B: Self = Self(0x0202020202020202);
    pub const FILE_C: Self = Self(0x0404040404040404);
    pub const FILE_D: Self = Self(0x0808080808080808);
    pub const FILE_E: Self = Self(0x1010101010101010);
    pub const FILE_F: Self = Self(0x2020202020202020);
    pub const FILE_G: Self = Self(0x4040404040404040);
    pub const FILE_H: Self = Self(0x8080808080808080);
    pub const RANK_1: Self = Self(0xFF);
    pub const RANK_2: Self = Self(0xFF00);
    pub const RANK_3: Self = Self(0xFF0000);
    pub const RANK_4: Self = Self(0xFF000000);
    pub const RANK_5: Self = Self(0xFF00000000);
    pub const RANK_6: Self = Self(0xFF0000000000);
    pub const RANK_7: Self = Self(0xFF000000000000);
    pub const RANK_8: Self = Self(0xFF00000000000000);

    pub fn contains(&self, square: &Square) -> bool {
        self.0 & (1u64 << square.as_bit_index()) != 0
    }

    pub fn set(&mut self, square: &Square) {
        self.0 |= 1 << square.as_bit_index();
    }

    pub fn clear(&mut self, square: &Square) {
        self.0 &= !(1u64 << square.as_bit_index());
    }

    pub fn pop_lsb(&mut self) {
        if self.0 == 0 {
            return;
        }
        self.0 &= self.0 - 1;
    }

    pub fn as_squares(&self) -> Vec<Square> {
        let mut res: Vec<Square> = vec![];
        let mut bb = *self;
        while bb.0 != 0 {
            let index = bb.0.trailing_zeros();
            bb.pop_lsb();
            res.push(Square::from_bit_index(index as u8));
        }

        res
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for file in (0..8).rev() {
            for rank in 0..8 {
                let n = file * 8 + rank;
                if (self.0 >> n) & 1 == 1 {
                    write!(f, "☒ ")?;
                } else {
                    write!(f, "□ ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::Shl for Bitboard {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << rhs.0)
    }
}

impl std::ops::Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl std::ops::Shr for Bitboard {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0 >> rhs.0)
    }
}

impl std::ops::Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_contains() {
        let white_pawns_bitboard = Bitboard(0xFF00);
        let black_pawns_bitboard = Bitboard(0xFF000000000000);
        assert!(white_pawns_bitboard.contains(&Square::from_algebraic_notation("b2").unwrap()));
        assert!(!white_pawns_bitboard.contains(&Square::from_algebraic_notation("c3").unwrap()));
        assert!(!black_pawns_bitboard.contains(&Square::from_algebraic_notation("b2").unwrap()));
    }
}
