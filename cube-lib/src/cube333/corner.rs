use super::{Face, StickerToPieceError};
use crate::TryFromIntToEnumError;

/// An enum for every corner piece location.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Corner {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

use Corner as C;

impl std::fmt::Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            C::UFR => write!(f, "UFR"),
            C::UFL => write!(f, "UFL"),
            C::UBL => write!(f, "UBL"),
            C::UBR => write!(f, "UBR"),
            C::DFR => write!(f, "DFR"),
            C::DFL => write!(f, "DFL"),
            C::DBL => write!(f, "DBL"),
            C::DBR => write!(f, "DBR"),
        }
    }
}

impl Corner {
    pub(crate) const ARRAY: [Corner; 8] = [
        C::UFR,
        C::UFL,
        C::UBL,
        C::UBR,
        C::DFR,
        C::DFL,
        C::DBL,
        C::DBR,
    ];
}

impl From<Corner> for u8 {
    fn from(value: Corner) -> Self {
        match value {
            C::UFR => 0,
            C::UFL => 1,
            C::UBL => 2,
            C::UBR => 3,
            C::DFR => 4,
            C::DFL => 5,
            C::DBL => 6,
            C::DBR => 7,
        }
    }
}

impl TryFrom<u8> for Corner {
    type Error = TryFromIntToEnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(C::UFR),
            1 => Ok(C::UFL),
            2 => Ok(C::UBL),
            3 => Ok(C::UBR),
            4 => Ok(C::DFR),
            5 => Ok(C::DFL),
            6 => Ok(C::DBL),
            7 => Ok(C::DBR),
            _ => Err(TryFromIntToEnumError::OutOfBounds),
        }
    }
}

/// An enum for every corner twist case.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum CornerTwist {
    Oriented,
    Clockwise,
    AntiClockwise,
}

use CornerTwist as CT;

impl From<CornerTwist> for u8 {
    fn from(value: CornerTwist) -> Self {
        match value {
            CornerTwist::Oriented => 0,
            CornerTwist::Clockwise => 1,
            CornerTwist::AntiClockwise => 2,
        }
    }
}

impl TryFrom<u8> for CornerTwist {
    type Error = TryFromIntToEnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CornerTwist::Oriented),
            1 => Ok(CornerTwist::Clockwise),
            2 => Ok(CornerTwist::AntiClockwise),
            _ => Err(TryFromIntToEnumError::OutOfBounds),
        }
    }
}

/// An enum to represent every corner sticker position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum CornerPos {
    UBL,
    UBR,
    UFR,
    UFL,
    LUB,
    LUF,
    LDF,
    LDB,
    FUL,
    FUR,
    FDR,
    FDL,
    RUF,
    RUB,
    RDB,
    RDF,
    BUR,
    BUL,
    BDL,
    BDR,
    DFR,
    DBR,
    DBL,
    DFL,
}

use CornerPos as CP;

impl CornerPos {
    pub(crate) fn clockwise(self) -> CornerPos {
        let (f1, f2, f3) = self.into();
        (f3, f1, f2).try_into().unwrap()
    }

    pub(crate) fn anticlockwise(self) -> CornerPos {
        let (f1, f2, f3) = self.into();
        (f2, f3, f1).try_into().unwrap()
    }

    /// Returns the corner piece that a corner sticker is on.
    /// TODO may need to re explain this.
    pub fn piece(self) -> Corner {
        match self {
            CP::UBL => Corner::UBL,
            CP::UBR => Corner::UBR,
            CP::UFR => Corner::UFR,
            CP::UFL => Corner::UFL,
            CP::LUB => Corner::UBL,
            CP::LUF => Corner::UFL,
            CP::LDF => Corner::DFL,
            CP::LDB => Corner::DBL,
            CP::FUL => Corner::UFL,
            CP::FUR => Corner::UFR,
            CP::FDR => Corner::DFR,
            CP::FDL => Corner::DFL,
            CP::RUF => Corner::UFR,
            CP::RUB => Corner::UBR,
            CP::RDB => Corner::DBR,
            CP::RDF => Corner::DFR,
            CP::BUR => Corner::UBR,
            CP::BUL => Corner::UBL,
            CP::BDL => Corner::DBL,
            CP::BDR => Corner::DBR,
            CP::DFR => Corner::DFR,
            CP::DBR => Corner::DBR,
            CP::DBL => Corner::DBL,
            CP::DFL => Corner::DFL,
        }
    }

    /// Returns the corner orientation of a sticker with respect to the U/D axis
    /// TODO may need to re explain this.
    pub fn ud_orientation(self) -> CornerTwist {
        match self {
            CP::UBL => CT::Oriented,
            CP::UBR => CT::Oriented,
            CP::UFR => CT::Oriented,
            CP::UFL => CT::Oriented,
            CP::LUB => CT::Clockwise,
            CP::LUF => CT::AntiClockwise,
            CP::LDF => CT::Clockwise,
            CP::LDB => CT::AntiClockwise,
            CP::FUL => CT::Clockwise,
            CP::FUR => CT::AntiClockwise,
            CP::FDR => CT::Clockwise,
            CP::FDL => CT::AntiClockwise,
            CP::RUF => CT::Clockwise,
            CP::RUB => CT::AntiClockwise,
            CP::RDB => CT::Clockwise,
            CP::RDF => CT::AntiClockwise,
            CP::BUR => CT::Clockwise,
            CP::BUL => CT::AntiClockwise,
            CP::BDL => CT::Clockwise,
            CP::BDR => CT::AntiClockwise,
            CP::DFR => CT::Oriented,
            CP::DBR => CT::Oriented,
            CP::DBL => CT::Oriented,
            CP::DFL => CT::Oriented,
        }
    }
}

impl From<Corner> for CornerPos {
    fn from(value: Corner) -> Self {
        match value {
            C::UFR => CP::UFR,
            C::UFL => CP::UFL,
            C::UBL => CP::UBL,
            C::UBR => CP::UBR,
            C::DFR => CP::DFR,
            C::DFL => CP::DFL,
            C::DBL => CP::DBL,
            C::DBR => CP::DBR,
        }
    }
}

impl From<(Corner, CornerTwist)> for CornerPos {
    fn from(value: (Corner, CornerTwist)) -> Self {
        match value {
            (C::UFR, CT::Oriented) => CP::UFR,
            (C::UFR, CT::Clockwise) => CP::RUF,
            (C::UFR, CT::AntiClockwise) => CP::FUR,
            (C::UFL, CT::Oriented) => CP::UFL,
            (C::UFL, CT::Clockwise) => CP::FUL,
            (C::UFL, CT::AntiClockwise) => CP::LUF,
            (C::UBL, CT::Oriented) => CP::UBL,
            (C::UBL, CT::Clockwise) => CP::LUB,
            (C::UBL, CT::AntiClockwise) => CP::BUL,
            (C::UBR, CT::Oriented) => CP::UBR,
            (C::UBR, CT::Clockwise) => CP::BUR,
            (C::UBR, CT::AntiClockwise) => CP::RUB,
            (C::DFR, CT::Oriented) => CP::DFR,
            (C::DFR, CT::Clockwise) => CP::FDR,
            (C::DFR, CT::AntiClockwise) => CP::RDF,
            (C::DFL, CT::Oriented) => CP::DFL,
            (C::DFL, CT::Clockwise) => CP::LDF,
            (C::DFL, CT::AntiClockwise) => CP::FDL,
            (C::DBL, CT::Oriented) => CP::DBL,
            (C::DBL, CT::Clockwise) => CP::BDL,
            (C::DBL, CT::AntiClockwise) => CP::LDB,
            (C::DBR, CT::Oriented) => CP::DBR,
            (C::DBR, CT::Clockwise) => CP::RDB,
            (C::DBR, CT::AntiClockwise) => CP::BDR,
        }
    }
}

impl TryFrom<(Face, Face, Face)> for CornerPos {
    type Error = StickerToPieceError;

    fn try_from(value: (Face, Face, Face)) -> Result<CornerPos, Self::Error> {
        use Face as F;
        match value {
            (F::U, F::L, F::B) => Ok(CP::UBL),
            (F::U, F::B, F::L) => Ok(CP::UBL),
            (F::U, F::R, F::B) => Ok(CP::UBR),
            (F::U, F::B, F::R) => Ok(CP::UBR),
            (F::U, F::R, F::F) => Ok(CP::UFR),
            (F::U, F::F, F::R) => Ok(CP::UFR),
            (F::U, F::L, F::F) => Ok(CP::UFL),
            (F::U, F::F, F::L) => Ok(CP::UFL),
            (F::L, F::B, F::U) => Ok(CP::LUB),
            (F::L, F::U, F::B) => Ok(CP::LUB),
            (F::L, F::F, F::U) => Ok(CP::LUF),
            (F::L, F::U, F::F) => Ok(CP::LUF),
            (F::L, F::F, F::D) => Ok(CP::LDF),
            (F::L, F::D, F::F) => Ok(CP::LDF),
            (F::L, F::B, F::D) => Ok(CP::LDB),
            (F::L, F::D, F::B) => Ok(CP::LDB),
            (F::F, F::L, F::U) => Ok(CP::FUL),
            (F::F, F::U, F::L) => Ok(CP::FUL),
            (F::F, F::R, F::U) => Ok(CP::FUR),
            (F::F, F::U, F::R) => Ok(CP::FUR),
            (F::F, F::R, F::D) => Ok(CP::FDR),
            (F::F, F::D, F::R) => Ok(CP::FDR),
            (F::F, F::L, F::D) => Ok(CP::FDL),
            (F::F, F::D, F::L) => Ok(CP::FDL),
            (F::R, F::F, F::U) => Ok(CP::RUF),
            (F::R, F::U, F::F) => Ok(CP::RUF),
            (F::R, F::B, F::U) => Ok(CP::RUB),
            (F::R, F::U, F::B) => Ok(CP::RUB),
            (F::R, F::B, F::D) => Ok(CP::RDB),
            (F::R, F::D, F::B) => Ok(CP::RDB),
            (F::R, F::F, F::D) => Ok(CP::RDF),
            (F::R, F::D, F::F) => Ok(CP::RDF),
            (F::B, F::R, F::U) => Ok(CP::BUR),
            (F::B, F::U, F::R) => Ok(CP::BUR),
            (F::B, F::L, F::U) => Ok(CP::BUL),
            (F::B, F::U, F::L) => Ok(CP::BUL),
            (F::B, F::L, F::D) => Ok(CP::BDL),
            (F::B, F::D, F::L) => Ok(CP::BDL),
            (F::B, F::R, F::D) => Ok(CP::BDR),
            (F::B, F::D, F::R) => Ok(CP::BDR),
            (F::D, F::L, F::F) => Ok(CP::DFL),
            (F::D, F::F, F::L) => Ok(CP::DFL),
            (F::D, F::R, F::F) => Ok(CP::DFR),
            (F::D, F::F, F::R) => Ok(CP::DFR),
            (F::D, F::R, F::B) => Ok(CP::DBR),
            (F::D, F::B, F::R) => Ok(CP::DBR),
            (F::D, F::L, F::B) => Ok(CP::DBL),
            (F::D, F::B, F::L) => Ok(CP::DBL),
            _ => Err(StickerToPieceError::InvalidPiece),
        }
    }
}

impl From<CornerPos> for (Face, Face, Face) {
    fn from(value: CornerPos) -> Self {
        use Face as F;
        match value {
            CP::UBL => (F::U, F::L, F::B),
            CP::UBR => (F::U, F::B, F::R),
            CP::UFR => (F::U, F::R, F::F),
            CP::UFL => (F::U, F::F, F::L),
            CP::LUB => (F::L, F::B, F::U),
            CP::LUF => (F::L, F::U, F::F),
            CP::LDF => (F::L, F::F, F::D),
            CP::LDB => (F::L, F::D, F::B),
            CP::FUL => (F::F, F::L, F::U),
            CP::FUR => (F::F, F::U, F::R),
            CP::FDR => (F::F, F::R, F::D),
            CP::FDL => (F::F, F::D, F::L),
            CP::RUF => (F::R, F::F, F::U),
            CP::RUB => (F::R, F::U, F::B),
            CP::RDB => (F::R, F::B, F::D),
            CP::RDF => (F::R, F::D, F::F),
            CP::BUR => (F::B, F::R, F::U),
            CP::BUL => (F::B, F::U, F::L),
            CP::BDL => (F::B, F::L, F::D),
            CP::BDR => (F::B, F::D, F::R),
            CP::DFR => (F::D, F::F, F::R),
            CP::DBR => (F::D, F::R, F::B),
            CP::DBL => (F::D, F::B, F::L),
            CP::DFL => (F::D, F::L, F::F),
        }
    }
}

impl From<CornerPos> for usize {
    fn from(value: CornerPos) -> Self {
        match value {
            CP::UBL => 0,
            CP::UBR => 1,
            CP::UFR => 2,
            CP::UFL => 3,
            CP::LUB => 4,
            CP::LUF => 5,
            CP::LDF => 6,
            CP::LDB => 7,
            CP::FUL => 8,
            CP::FUR => 9,
            CP::FDR => 10,
            CP::FDL => 11,
            CP::RUF => 12,
            CP::RUB => 13,
            CP::RDB => 14,
            CP::RDF => 15,
            CP::BUR => 16,
            CP::BUL => 17,
            CP::BDL => 18,
            CP::BDR => 19,
            CP::DFR => 20,
            CP::DBR => 21,
            CP::DBL => 22,
            CP::DFL => 23,
        }
    }
}

impl From<CornerPos> for Face {
    fn from(value: CornerPos) -> Self {
        use Face as F;
        match value {
            CP::UBL => F::U,
            CP::UBR => F::U,
            CP::UFR => F::U,
            CP::UFL => F::U,
            CP::LUB => F::L,
            CP::LUF => F::L,
            CP::LDF => F::L,
            CP::LDB => F::L,
            CP::FUL => F::F,
            CP::FUR => F::F,
            CP::FDR => F::F,
            CP::FDL => F::F,
            CP::RUF => F::R,
            CP::RUB => F::R,
            CP::RDB => F::R,
            CP::RDF => F::R,
            CP::BUR => F::B,
            CP::BUL => F::B,
            CP::BDL => F::B,
            CP::BDR => F::B,
            CP::DFR => F::D,
            CP::DBR => F::D,
            CP::DBL => F::D,
            CP::DFL => F::D,
        }
    }
}
