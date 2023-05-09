use super::{Face, StateConversionError};

/// An enum for every edge piece location.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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
    pub(crate) const ARRAY: [Edge; 12] = [UF, UL, UB, UR, DF, DL, DB, DR, FR, FL, BL, BR];
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

/// An enum to tell if an edge is flipped or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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

/// An enum to represent every edge sticker position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
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
    pub(crate) fn flip(self) -> EdgePos {
        let (f1, f2) = self.into();
        (f2, f1).try_into().unwrap()
    }

    pub(crate) fn piece(self) -> Edge {
        use EdgePos::*;
        match self {
            UB => Edge::UB,
            UR => Edge::UR,
            UF => Edge::UF,
            UL => Edge::UL,
            LU => Edge::UL,
            LF => Edge::FL,
            LD => Edge::DL,
            LB => Edge::BL,
            FU => Edge::UF,
            FR => Edge::FR,
            FD => Edge::DF,
            FL => Edge::FL,
            RU => Edge::UR,
            RB => Edge::BR,
            RD => Edge::DR,
            RF => Edge::FR,
            BU => Edge::UB,
            BL => Edge::BL,
            BD => Edge::DB,
            BR => Edge::BR,
            DF => Edge::DF,
            DR => Edge::DR,
            DB => Edge::DB,
            DL => Edge::DL,
        }
    }

    /// Returns the eo of a piece with respect to the F/B axis.
    /// TODO! may need to reexplain this + implement eo for other axis.
    pub fn fb_orientation(self) -> EdgeFlip {
        use EdgeFlip::*;
        use EdgePos::*;
        match self {
            UB => Oriented,
            UR => Oriented,
            UF => Oriented,
            UL => Oriented,
            LU => Flipped,
            LF => Flipped,
            LD => Flipped,
            LB => Flipped,
            FU => Flipped,
            FR => Oriented,
            FD => Flipped,
            FL => Oriented,
            RU => Flipped,
            RB => Flipped,
            RD => Flipped,
            RF => Flipped,
            BU => Flipped,
            BL => Oriented,
            BD => Flipped,
            BR => Oriented,
            DF => Oriented,
            DR => Oriented,
            DB => Oriented,
            DL => Oriented,
        }
    }
}

impl From<Edge> for EdgePos {
    fn from(value: Edge) -> Self {
        use EdgePos::*;
        match value {
            Edge::UF => UF,
            Edge::UL => UL,
            Edge::UB => UB,
            Edge::UR => UR,
            Edge::DF => DF,
            Edge::DL => DL,
            Edge::DB => DB,
            Edge::DR => DR,
            Edge::FR => FR,
            Edge::FL => FL,
            Edge::BL => BL,
            Edge::BR => BR,
        }
    }
}

impl From<(Edge, EdgeFlip)> for EdgePos {
    fn from(value: (Edge, EdgeFlip)) -> Self {
        use EdgePos::*;
        match value {
            (Edge::UF, EdgeFlip::Oriented) => UF,
            (Edge::UF, EdgeFlip::Flipped) => FU,
            (Edge::UL, EdgeFlip::Oriented) => UL,
            (Edge::UL, EdgeFlip::Flipped) => LU,
            (Edge::UB, EdgeFlip::Oriented) => UB,
            (Edge::UB, EdgeFlip::Flipped) => BU,
            (Edge::UR, EdgeFlip::Oriented) => UR,
            (Edge::UR, EdgeFlip::Flipped) => RU,
            (Edge::DF, EdgeFlip::Oriented) => DF,
            (Edge::DF, EdgeFlip::Flipped) => FD,
            (Edge::DL, EdgeFlip::Oriented) => DL,
            (Edge::DL, EdgeFlip::Flipped) => LD,
            (Edge::DB, EdgeFlip::Oriented) => DB,
            (Edge::DB, EdgeFlip::Flipped) => BD,
            (Edge::DR, EdgeFlip::Oriented) => DR,
            (Edge::DR, EdgeFlip::Flipped) => RD,
            (Edge::FR, EdgeFlip::Oriented) => FR,
            (Edge::FR, EdgeFlip::Flipped) => RF,
            (Edge::FL, EdgeFlip::Oriented) => FL,
            (Edge::FL, EdgeFlip::Flipped) => LF,
            (Edge::BL, EdgeFlip::Oriented) => BL,
            (Edge::BL, EdgeFlip::Flipped) => LB,
            (Edge::BR, EdgeFlip::Oriented) => BR,
            (Edge::BR, EdgeFlip::Flipped) => RB,
        }
    }
}

impl TryFrom<(Face, Face)> for EdgePos {
    type Error = ();

    fn try_from(value: (Face, Face)) -> Result<EdgePos, Self::Error> {
        use EdgePos::*;
        use Face::*;
        match value {
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

impl From<EdgePos> for (Face, Face) {
    fn from(value: EdgePos) -> Self {
        use EdgePos::*;
        use Face::*;
        match value {
            UB => (U, B),
            UR => (U, R),
            UF => (U, F),
            UL => (U, L),
            LU => (L, U),
            LF => (L, F),
            LD => (L, D),
            LB => (L, B),
            FU => (F, U),
            FR => (F, R),
            FD => (F, D),
            FL => (F, L),
            RU => (R, U),
            RB => (R, B),
            RD => (R, D),
            RF => (R, F),
            BU => (B, U),
            BL => (B, L),
            BD => (B, D),
            BR => (B, R),
            DF => (D, F),
            DR => (D, R),
            DB => (D, B),
            DL => (D, L),
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
