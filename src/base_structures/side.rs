use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Side(u8);
impl Side {
    pub const WHITE: Self = Self(0);
    pub const BLACK: Self = Self(1);

    #[inline]
    pub const fn from_raw(value: u8) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn get_value(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn get_flipped_value(&self) -> u8 {
        1 - self.0
    }

    #[inline]
    pub const fn flipped(&self) -> Self {
        Self(self.get_flipped_value())
    }

    #[inline]
    pub fn mut_flip(&mut self) {
        self.0 = 1 - self.0;
    }

    pub fn to_string(&self) -> String {
        if *self == Side::WHITE {
            "White".to_string()
        } else {
            "Black".to_string()
        }
    }
}

impl Display for Side {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.to_string())
    }
}

impl From<Side> for u8 {
    fn from(side: Side) -> Self {
        side.get_value()
    }
}

impl From<Side> for usize {
    fn from(side: Side) -> Self {
        side.get_value() as usize
    }
}
