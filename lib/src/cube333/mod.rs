// Cube333 module

pub mod coordcube;
pub mod coordinate;
pub mod moves;
pub mod solver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubieCube {
    pub co: [u8; 8],
    pub cp: [u8; 8],
    pub eo: [u8; 12],
    pub ep: [u8; 12],
}

impl CubieCube {
    pub fn solved() -> Self {
        CubieCube {
            co: [0; 8],
            cp: Corners::ARRAY.map(|c| usize::from(c) as u8),
            eo: [0; 12],
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
