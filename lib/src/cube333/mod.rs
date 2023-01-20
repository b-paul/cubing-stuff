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

use Sticker::*;
impl StickerCube {
    pub const SOLVED: StickerCube = StickerCube {
        edges: [
            S1, S1, S1, S1, S2, S2, S2, S2, S3, S3, S3, S3, S4, S4, S4, S4, S5, S5, S5, S5, S6, S6,
            S6, S6,
        ],
        corners: [
            S1, S1, S1, S1, S2, S2, S2, S2, S3, S3, S3, S3, S4, S4, S4, S4, S5, S5, S5, S5, S6, S6,
            S6, S6,
        ],
        centers: [S1, S2, S3, S4, S5, S6],
    };

    pub fn sticker_to_face(&self, sticker: Sticker) -> Face {
        self.centers
            .iter()
            .position(|c| *c == sticker)
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub fn edge_at(&self, pos: EdgePos) -> Result<EdgePos, ()> {
        let s1 = self[pos];
        let s2 = self[pos.flip()];
        let f1 = self.sticker_to_face(s1);
        let f2 = self.sticker_to_face(s2);
        (f1, f2).try_into()
    }
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

impl EdgePos {
    pub fn flip(self) -> EdgePos {
        use EdgePos::*;
        match self {
            UB => BU,
            UR => RU,
            UF => FU,
            UL => LU,
            LU => UL,
            LF => FL,
            LD => DL,
            LB => BL,
            FU => UF,
            FR => RF,
            FD => DF,
            FL => LF,
            RU => UR,
            RB => BR,
            RD => DR,
            RF => FR,
            BU => UB,
            BL => LB,
            BD => DB,
            BR => RB,
            DF => FD,
            DR => RD,
            DB => BD,
            DL => LD,
        }
    }
}

impl TryInto<EdgePos> for (Face, Face) {
    type Error = ();

    fn try_into(self) -> Result<EdgePos, Self::Error> {
        use EdgePos::*;
        use Face::*;
        match self {
            (U, L) => Ok(UL),
            (U, F) => Ok(UF),
            (U, R) => Ok(UR),
            (U, B) => Ok(UB),
            (L, U) => Ok(LU),
            (L, F) => Ok(LF),
            (L, B) => Ok(LB),
            (L, D) => Ok(LD),
            (F, U) => Ok(FU),
            (F, L) => Ok(FL),
            (F, R) => Ok(FR),
            (F, D) => Ok(FD),
            (R, U) => Ok(RU),
            (R, F) => Ok(RF),
            (R, B) => Ok(RB),
            (R, D) => Ok(RD),
            (B, U) => Ok(BU),
            (B, L) => Ok(BL),
            (B, R) => Ok(BR),
            (B, D) => Ok(BD),
            (D, L) => Ok(DL),
            (D, F) => Ok(DF),
            (D, R) => Ok(DR),
            (D, B) => Ok(DB),
            _ => Err(()),
        }
    }
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
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
}

impl TryFrom<usize> for Face {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use Face::*;
        match value {
            0 => Ok(U),
            1 => Ok(L),
            2 => Ok(F),
            3 => Ok(R),
            4 => Ok(B),
            5 => Ok(D),
            _ => Err(()),
        }
    }
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

impl From<EdgePos> for Face {
    fn from(value: EdgePos) -> Self {
        use EdgePos::*;
        use Face::*;
        match value {
            UB => U,
            UR => U,
            UF => U,
            UL => U,
            LU => L,
            LF => L,
            LD => L,
            LB => L,
            FU => F,
            FR => F,
            FD => F,
            FL => F,
            RU => R,
            RB => R,
            RD => R,
            RF => R,
            BU => B,
            BL => B,
            BD => B,
            BR => B,
            DF => D,
            DR => D,
            DB => D,
            DL => D,
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

#[cfg(test)]
mod tests {
    #[test]
    fn pieces_on_solved_cube() {
        use super::EdgePos::*;
        use super::StickerCube;
        assert_eq!(StickerCube::SOLVED.edge_at(UB).unwrap(), UB, "UB");
        assert_eq!(StickerCube::SOLVED.edge_at(UR).unwrap(), UR, "UR");
        assert_eq!(StickerCube::SOLVED.edge_at(UF).unwrap(), UF, "UF");
        assert_eq!(StickerCube::SOLVED.edge_at(UL).unwrap(), UL, "UL");
        assert_eq!(StickerCube::SOLVED.edge_at(LU).unwrap(), LU, "LU");
        assert_eq!(StickerCube::SOLVED.edge_at(LF).unwrap(), LF, "LF");
        assert_eq!(StickerCube::SOLVED.edge_at(LD).unwrap(), LD, "LD");
        assert_eq!(StickerCube::SOLVED.edge_at(LB).unwrap(), LB, "LB");
        assert_eq!(StickerCube::SOLVED.edge_at(FU).unwrap(), FU, "FU");
        assert_eq!(StickerCube::SOLVED.edge_at(FR).unwrap(), FR, "FR");
        assert_eq!(StickerCube::SOLVED.edge_at(FD).unwrap(), FD, "FD");
        assert_eq!(StickerCube::SOLVED.edge_at(FL).unwrap(), FL, "FL");
        assert_eq!(StickerCube::SOLVED.edge_at(RU).unwrap(), RU, "RU");
        assert_eq!(StickerCube::SOLVED.edge_at(RB).unwrap(), RB, "RB");
        assert_eq!(StickerCube::SOLVED.edge_at(RD).unwrap(), RD, "RD");
        assert_eq!(StickerCube::SOLVED.edge_at(RF).unwrap(), RF, "RF");
        assert_eq!(StickerCube::SOLVED.edge_at(BU).unwrap(), BU, "BU");
        assert_eq!(StickerCube::SOLVED.edge_at(BL).unwrap(), BL, "BL");
        assert_eq!(StickerCube::SOLVED.edge_at(BD).unwrap(), BD, "BD");
        assert_eq!(StickerCube::SOLVED.edge_at(BR).unwrap(), BR, "BR");
        assert_eq!(StickerCube::SOLVED.edge_at(DF).unwrap(), DF, "DF");
        assert_eq!(StickerCube::SOLVED.edge_at(DR).unwrap(), DR, "DR");
        assert_eq!(StickerCube::SOLVED.edge_at(DB).unwrap(), DB, "DB");
        assert_eq!(StickerCube::SOLVED.edge_at(DL).unwrap(), DL, "DL");
    }
}
