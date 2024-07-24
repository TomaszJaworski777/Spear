use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct CastleRight(u8);
impl CastleRight {
    pub const WHITE_QUEEN: Self = Self(0b1000);
    pub const WHITE_KING: Self = Self(0b0100);
    pub const BLACK_QUEEN: Self = Self(0b0010);
    pub const BLACK_KING: Self = Self(0b0001);
    pub const NULL: Self = Self(0);

    pub const ROOK_MASKS: [u8; 64] = {
        let mut result = [0; 64];
        result[0] = 0b1000;
        result[4] = 0b1100;
        result[7] = 0b0100;
        result[56] = 0b0010;
        result[60] = 0b0011;
        result[63] = 0b0001;
        result
    };

    pub const ROOK_POSITIONS: [u8; 2] = [0, 7];

    #[inline]
    pub fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    #[inline]
    pub(crate) fn set_right(&mut self, right: CastleRight) {
        self.0 |= right.0
    }

    #[inline]
    pub(crate) fn remove_right(&mut self, right: CastleRight) {
        self.0 &= !right.0
    }

    #[inline]
    pub fn has_right(&self, right: CastleRight) -> bool {
        self.0 & right.0 > 0
    }

    #[inline]
    pub fn get_value(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn get_index(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    pub fn to_string(&self) -> String {
        let mut rights = "".to_string();
        if self.has_right(CastleRight::WHITE_KING) {
            rights += "K";
        }
        if self.has_right(CastleRight::WHITE_QUEEN) {
            rights += "Q";
        }
        if self.has_right(CastleRight::BLACK_KING) {
            rights += "k";
        }
        if self.has_right(CastleRight::BLACK_QUEEN) {
            rights += "q";
        }
        if rights == "" {
            rights = "-".to_string();
        }
        rights
    }
}

impl Display for CastleRight {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.to_string())
    }
}

impl From<CastleRight> for u8 {
    fn from(castle_right: CastleRight) -> Self {
        castle_right.get_value()
    }
}

impl From<CastleRight> for usize {
    fn from(castile_right: CastleRight) -> Self {
        castile_right.get_value() as usize
    }
}
