// Cube333 module

pub mod coordcube;
pub mod moves;
pub mod solver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubieCube {
    pub co: [CornerTwist; 8],
    pub cp: [Corner; 8],
    pub eo: [EdgeFlip; 12],
    pub ep: [Edge; 12],
}

impl CubieCube {
    pub const SOLVED: CubieCube = CubieCube {
        co: [CornerTwist::Oriented; 8],
        cp: Corner::ARRAY,
        eo: [EdgeFlip::Oriented; 12],
        ep: Edge::ARRAY,
    };
}

#[derive(Debug)]
pub enum StateConversionError {
    InvalidValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

use Corner::*;

impl Corner {
    pub const ARRAY: [Corner; 8] = [UFR, UFL, UBL, UBR, DFR, DFL, DBL, DBR];
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Edge {
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

use Edge::*;

impl Edge {
    pub const ARRAY: [Edge; 12] = [UF, UL, UB, UR, DF, DL, DB, DR, FR, FL, BL, BR];
}

impl From<Edge> for u8 {
    fn from(value: Edge) -> Self {
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

impl TryFrom<u8> for Edge {
    type Error = StateConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UF),
            1 => Ok(UL),
            2 => Ok(UB),
            3 => Ok(UR),
            4 => Ok(DF),
            5 => Ok(DL),
            6 => Ok(DB),
            7 => Ok(DR),
            8 => Ok(FR),
            9 => Ok(FL),
            10 => Ok(BL),
            11 => Ok(BR),
            _ => Err(StateConversionError::InvalidValue),
        }
    }
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
    type Error = StateConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EdgeFlip::Oriented),
            1 => Ok(EdgeFlip::Flipped),
            _ => Err(StateConversionError::InvalidValue),
        }
    }
}

pub struct StickerCube {
    pub edges: [Sticker; 24],
    pub corners: [Sticker; 24],
    pub centers: [Sticker; 6],
}

impl StickerCube {
    // :grimacing:
    pub const SOLVED: StickerCube = StickerCube {
        edges: [
            Sticker::S1,
            Sticker::S1,
            Sticker::S1,
            Sticker::S1,
            Sticker::S2,
            Sticker::S2,
            Sticker::S2,
            Sticker::S2,
            Sticker::S3,
            Sticker::S3,
            Sticker::S3,
            Sticker::S3,
            Sticker::S4,
            Sticker::S4,
            Sticker::S4,
            Sticker::S4,
            Sticker::S5,
            Sticker::S5,
            Sticker::S5,
            Sticker::S5,
            Sticker::S6,
            Sticker::S6,
            Sticker::S6,
            Sticker::S6,
        ],
        corners: [
            Sticker::S1,
            Sticker::S1,
            Sticker::S1,
            Sticker::S1,
            Sticker::S2,
            Sticker::S2,
            Sticker::S2,
            Sticker::S2,
            Sticker::S3,
            Sticker::S3,
            Sticker::S3,
            Sticker::S3,
            Sticker::S4,
            Sticker::S4,
            Sticker::S4,
            Sticker::S4,
            Sticker::S5,
            Sticker::S5,
            Sticker::S5,
            Sticker::S5,
            Sticker::S6,
            Sticker::S6,
            Sticker::S6,
            Sticker::S6,
        ],
        centers: [
            Sticker::S1,
            Sticker::S2,
            Sticker::S3,
            Sticker::S4,
            Sticker::S5,
            Sticker::S6,
        ],
    };
}

use std::ops::Index;

impl Index<EdgePos> for StickerCube {
    type Output = Sticker;

    fn index(&self, index: EdgePos) -> &Self::Output {
        &self.edges[index as usize]
    }
}

impl Index<CornerPos> for StickerCube {
    type Output = Sticker;

    fn index(&self, index: CornerPos) -> &Self::Output {
        &self.corners[index as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EdgePos {
    UB,
    UR,
    UF,
    UL,
    LU,
    LF,
    LD,
    LB,
    FU,
    FR,
    FD,
    FL,
    RU,
    RB,
    RD,
    RF,
    BU,
    BL,
    BD,
    BR,
    DF,
    DR,
    DB,
    DL,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sticker {
    /// U
    S1,
    /// L
    S2,
    /// F
    S3,
    /// R
    S4,
    /// B
    S5,
    /// D
    S6,
}

impl From<EdgePos> for usize {
    fn from(value: EdgePos) -> Self {
        use EdgePos::*;
        match value {
            UB => 0,
            UR => 1,
            UF => 2,
            UL => 3,
            LU => 4,
            LF => 5,
            LD => 6,
            LB => 7,
            FU => 8,
            FR => 9,
            FD => 10,
            FL => 11,
            RU => 12,
            RB => 13,
            RD => 14,
            RF => 15,
            BU => 16,
            BL => 17,
            BD => 18,
            BR => 19,
            DF => 20,
            DR => 21,
            DB => 22,
            DL => 23,
        }
    }
}

impl From<EdgePos> for Sticker {
    fn from(value: EdgePos) -> Self {
        use EdgePos::*;
        use Sticker::*;
        match value {
            UB => S1,
            UR => S1,
            UF => S1,
            UL => S1,
            LU => S2,
            LF => S2,
            LD => S2,
            LB => S2,
            FU => S3,
            FR => S3,
            FD => S3,
            FL => S3,
            RU => S4,
            RB => S4,
            RD => S4,
            RF => S4,
            BU => S5,
            BL => S5,
            BD => S5,
            BR => S5,
            DF => S6,
            DR => S6,
            DB => S6,
            DL => S6,
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

impl From<CornerPos> for Sticker {
    fn from(value: CornerPos) -> Self {
        use CornerPos::*;
        use Sticker::*;
        match value {
            UBL => S1,
            UBR => S1,
            UFR => S1,
            UFL => S1,
            LUB => S2,
            LUF => S2,
            LDF => S2,
            LDB => S2,
            FUL => S3,
            FUR => S3,
            FDR => S3,
            FDL => S3,
            RUF => S4,
            RUB => S4,
            RDB => S4,
            RDF => S4,
            BUR => S5,
            BUL => S5,
            BDL => S5,
            BDR => S5,
            DFR => S6,
            DBR => S6,
            DBL => S6,
            DFL => S6,
        }
    }
}
