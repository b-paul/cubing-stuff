// Cube333 module

pub mod coordcube;
pub mod moves;
pub mod solver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubieCube {
    pub co: [CornerTwist; 8],
    pub cp: [u8; 8],
    pub eo: [EdgeFlip; 12],
    pub ep: [u8; 12],
}

impl CubieCube {
    pub fn solved() -> Self {
        CubieCube {
            co: [CornerTwist::Oriented; 8],
            cp: Corners::ARRAY.map(|c| usize::from(c) as u8),
            eo: [EdgeFlip::Oriented; 12],
            ep: Edges::ARRAY.map(|e| usize::from(e) as u8),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Corners {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

use Corners::*;

impl Corners {
    pub const ARRAY: [Corners; 8] = [UFR, UFL, UBL, UBR, DFR, DFL, DBL, DBR];
}

impl From<Corners> for usize {
    fn from(value: Corners) -> Self {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Edges {
    UF,
    UL,
    UB,
    UR,
    DF,
    DL,
    DB,
    DR,
    FR,
    FL,
    BL,
    BR,
}

use Edges::*;

impl Edges {
    pub const ARRAY: [Edges; 12] = [UF, UL, UB, UR, DF, DL, DB, DR, FR, FL, BL, BR];
}

impl From<Edges> for usize {
    fn from(value: Edges) -> Self {
        match value {
            UF => 0,
            UL => 1,
            UB => 2,
            UR => 3,
            DF => 4,
            DL => 5,
            DB => 6,
            DR => 7,
            FR => 8,
            FL => 9,
            BL => 10,
            BR => 11,
        }
    }
}

#[derive(Debug)]
pub enum OrientationConversionError {
    InvalidOrientationValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    type Error = OrientationConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CornerTwist::Oriented),
            1 => Ok(CornerTwist::Clockwise),
            2 => Ok(CornerTwist::AntiClockwise),
            _ => Err(OrientationConversionError::InvalidOrientationValue),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EdgeFlip {
    Oriented,
    Flipped,
}

impl From<EdgeFlip> for u8 {
    fn from(value: EdgeFlip) -> Self {
        match value {
            EdgeFlip::Oriented => 0,
            EdgeFlip::Flipped => 1,
        }
    }
}

impl TryFrom<u8> for EdgeFlip {
    type Error = OrientationConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EdgeFlip::Oriented),
            1 => Ok(EdgeFlip::Flipped),
            _ => Err(OrientationConversionError::InvalidOrientationValue),
        }
    }
}
