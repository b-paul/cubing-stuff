use super::{Face, StateConversionError};

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

impl Corner {
    pub(crate) const ARRAY: [Corner; 8] = [UFR, UFL, UBL, UBR, DFR, DFL, DBL, DBR];
}

use Corner::*;

impl From<Corner> for u8 {
    fn from(value: Corner) -> Self {
        match value {
            UFR => 0,
            UFL => 1,
            UBL => 2,
            UBR => 3,
            DFR => 4,
            DFL => 5,
            DBL => 6,
            DBR => 7,
        }
    }
}

impl TryFrom<u8> for Corner {
    type Error = StateConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UFR),
            1 => Ok(UFL),
            2 => Ok(UBL),
            3 => Ok(UBR),
            4 => Ok(DFR),
            5 => Ok(DFL),
            6 => Ok(DBL),
            7 => Ok(DBR),
            _ => Err(StateConversionError::InvalidValue),
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
    type Error = StateConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CornerTwist::Oriented),
            1 => Ok(CornerTwist::Clockwise),
            2 => Ok(CornerTwist::AntiClockwise),
            _ => Err(StateConversionError::InvalidValue),
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
        use CornerPos::*;
        match self {
            UBL => Corner::UBL,
            UBR => Corner::UBR,
            UFR => Corner::UFR,
            UFL => Corner::UFL,
            LUB => Corner::UBL,
            LUF => Corner::UFL,
            LDF => Corner::DFL,
            LDB => Corner::DBL,
            FUL => Corner::UFL,
            FUR => Corner::UFR,
            FDR => Corner::DFR,
            FDL => Corner::DFL,
            RUF => Corner::UFR,
            RUB => Corner::UBR,
            RDB => Corner::DBR,
            RDF => Corner::DFR,
            BUR => Corner::UBR,
            BUL => Corner::UBL,
            BDL => Corner::DBL,
            BDR => Corner::DBR,
            DFR => Corner::DFR,
            DBR => Corner::DBR,
            DBL => Corner::DBL,
            DFL => Corner::DFL,
        }
    }

    /// Returns the corner orientation of a sticker with respect to the U/D axis
    /// TODO may need to re explain this.
    pub fn ud_orientation(self) -> CornerTwist {
        use CornerPos::*;
        use CornerTwist::*;
        match self {
            UBL => Oriented,
            UBR => Oriented,
            UFR => Oriented,
            UFL => Oriented,
            LUB => Clockwise,
            LUF => AntiClockwise,
            LDF => Clockwise,
            LDB => AntiClockwise,
            FUL => Clockwise,
            FUR => AntiClockwise,
            FDR => Clockwise,
            FDL => AntiClockwise,
            RUF => Clockwise,
            RUB => AntiClockwise,
            RDB => Clockwise,
            RDF => AntiClockwise,
            BUR => Clockwise,
            BUL => AntiClockwise,
            BDL => Clockwise,
            BDR => AntiClockwise,
            DFR => Oriented,
            DBR => Oriented,
            DBL => Oriented,
            DFL => Oriented,
        }
    }
}

impl From<Corner> for CornerPos {
    fn from(value: Corner) -> Self {
        use CornerPos::*;
        match value {
            Corner::UFR => UFR,
            Corner::UFL => UFL,
            Corner::UBL => UBL,
            Corner::UBR => UBR,
            Corner::DFR => DFR,
            Corner::DFL => DFL,
            Corner::DBL => DBL,
            Corner::DBR => DBR,
        }
    }
}

impl From<(Corner, CornerTwist)> for CornerPos {
    fn from(value: (Corner, CornerTwist)) -> Self {
        use CornerPos::*;
        match value {
            (Corner::UFR, CornerTwist::Oriented) => UFR,
            (Corner::UFR, CornerTwist::Clockwise) => RUF,
            (Corner::UFR, CornerTwist::AntiClockwise) => FUR,
            (Corner::UFL, CornerTwist::Oriented) => UFL,
            (Corner::UFL, CornerTwist::Clockwise) => FUL,
            (Corner::UFL, CornerTwist::AntiClockwise) => LUF,
            (Corner::UBL, CornerTwist::Oriented) => UBL,
            (Corner::UBL, CornerTwist::Clockwise) => LUB,
            (Corner::UBL, CornerTwist::AntiClockwise) => BUL,
            (Corner::UBR, CornerTwist::Oriented) => UBR,
            (Corner::UBR, CornerTwist::Clockwise) => BUR,
            (Corner::UBR, CornerTwist::AntiClockwise) => RUB,
            (Corner::DFR, CornerTwist::Oriented) => DFR,
            (Corner::DFR, CornerTwist::Clockwise) => FDR,
            (Corner::DFR, CornerTwist::AntiClockwise) => RDF,
            (Corner::DFL, CornerTwist::Oriented) => DFL,
            (Corner::DFL, CornerTwist::Clockwise) => LDF,
            (Corner::DFL, CornerTwist::AntiClockwise) => FDL,
            (Corner::DBL, CornerTwist::Oriented) => DBL,
            (Corner::DBL, CornerTwist::Clockwise) => BDL,
            (Corner::DBL, CornerTwist::AntiClockwise) => LDB,
            (Corner::DBR, CornerTwist::Oriented) => DBR,
            (Corner::DBR, CornerTwist::Clockwise) => RDB,
            (Corner::DBR, CornerTwist::AntiClockwise) => BDR,
        }
    }
}

impl TryFrom<(Face, Face, Face)> for CornerPos {
    type Error = ();

    fn try_from(value: (Face, Face, Face)) -> Result<CornerPos, Self::Error> {
        use CornerPos::*;
        use Face::*;
        match value {
            (U, L, B) => Ok(UBL),
            (U, B, L) => Ok(UBL),
            (U, R, B) => Ok(UBR),
            (U, B, R) => Ok(UBR),
            (U, R, F) => Ok(UFR),
            (U, F, R) => Ok(UFR),
            (U, L, F) => Ok(UFL),
            (U, F, L) => Ok(UFL),
            (L, B, U) => Ok(LUB),
            (L, U, B) => Ok(LUB),
            (L, F, U) => Ok(LUF),
            (L, U, F) => Ok(LUF),
            (L, F, D) => Ok(LDF),
            (L, D, F) => Ok(LDF),
            (L, B, D) => Ok(LDB),
            (L, D, B) => Ok(LDB),
            (F, L, U) => Ok(FUL),
            (F, U, L) => Ok(FUL),
            (F, R, U) => Ok(FUR),
            (F, U, R) => Ok(FUR),
            (F, R, D) => Ok(FDR),
            (F, D, R) => Ok(FDR),
            (F, L, D) => Ok(FDL),
            (F, D, L) => Ok(FDL),
            (R, F, U) => Ok(RUF),
            (R, U, F) => Ok(RUF),
            (R, B, U) => Ok(RUB),
            (R, U, B) => Ok(RUB),
            (R, B, D) => Ok(RDB),
            (R, D, B) => Ok(RDB),
            (R, F, D) => Ok(RDF),
            (R, D, F) => Ok(RDF),
            (B, R, U) => Ok(BUR),
            (B, U, R) => Ok(BUR),
            (B, L, U) => Ok(BUL),
            (B, U, L) => Ok(BUL),
            (B, L, D) => Ok(BDL),
            (B, D, L) => Ok(BDL),
            (B, R, D) => Ok(BDR),
            (B, D, R) => Ok(BDR),
            (D, L, F) => Ok(DFL),
            (D, F, L) => Ok(DFL),
            (D, R, F) => Ok(DFR),
            (D, F, R) => Ok(DFR),
            (D, R, B) => Ok(DBR),
            (D, B, R) => Ok(DBR),
            (D, L, B) => Ok(DBL),
            (D, B, L) => Ok(DBL),
            _ => Err(()),
        }
    }
}

impl From<CornerPos> for (Face, Face, Face) {
    fn from(value: CornerPos) -> Self {
        use CornerPos::*;
        use Face::*;
        match value {
            UBL => (U, L, B),
            UBR => (U, B, R),
            UFR => (U, R, F),
            UFL => (U, F, L),
            LUB => (L, B, U),
            LUF => (L, U, F),
            LDF => (L, F, D),
            LDB => (L, D, B),
            FUL => (F, L, U),
            FUR => (F, U, R),
            FDR => (F, R, D),
            FDL => (F, D, L),
            RUF => (R, F, U),
            RUB => (R, U, B),
            RDB => (R, B, D),
            RDF => (R, D, F),
            BUR => (B, R, U),
            BUL => (B, U, L),
            BDL => (B, L, D),
            BDR => (B, D, R),
            DFR => (D, F, R),
            DBR => (D, R, B),
            DBL => (D, B, L),
            DFL => (D, L, F),
        }
    }
}

impl From<CornerPos> for usize {
    fn from(value: CornerPos) -> Self {
        use CornerPos::*;
        match value {
            UBL => 0,
            UBR => 1,
            UFR => 2,
            UFL => 3,
            LUB => 4,
            LUF => 5,
            LDF => 6,
            LDB => 7,
            FUL => 8,
            FUR => 9,
            FDR => 10,
            FDL => 11,
            RUF => 12,
            RUB => 13,
            RDB => 14,
            RDF => 15,
            BUR => 16,
            BUL => 17,
            BDL => 18,
            BDR => 19,
            DFR => 20,
            DBR => 21,
            DBL => 22,
            DFL => 23,
        }
    }
}

impl From<CornerPos> for Face {
    fn from(value: CornerPos) -> Self {
        use CornerPos::*;
        use Face::*;
        match value {
            UBL => U,
            UBR => U,
            UFR => U,
            UFL => U,
            LUB => L,
            LUF => L,
            LDF => L,
            LDB => L,
            FUL => F,
            FUR => F,
            FDR => F,
            FDL => F,
            RUF => R,
            RUB => R,
            RDB => R,
            RDF => R,
            BUR => B,
            BUL => B,
            BDL => B,
            BDR => B,
            DFR => D,
            DBR => D,
            DBL => D,
            DFL => D,
        }
    }
}
