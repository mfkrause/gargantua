use std::fmt::Display;

use crate::game::game_state::Square;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(!0);

    pub fn contains(self, square: &Square) -> bool {
        self.0 & (1u64 << square.as_bit_index()) != 0
    }

    pub fn set(&mut self, square: &Square) {
        self.0 |= 1 << square.as_bit_index();
    }

    pub fn clear(&mut self, square: &Square) {
        self.0 &= !(1u64 << square.as_bit_index());
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
