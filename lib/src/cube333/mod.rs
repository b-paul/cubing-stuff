// Cube333 module

pub mod coordcube;
pub mod coordinate;
pub mod moves;
pub mod solver;

// TODO
// generalized coordinates (user defined coordinates)

#[derive(Debug, PartialEq, Clone)]
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
            cp: [0; 8],
            eo: [0; 12],
            ep: [0; 12],
        }
    }
}

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
pub enum Edges {
    UF,
    UL,
    UB,
    UR,
    FR,
    FL,
    BL,
    BR,
    DF,
    DL,
    DB,
    DR,
}
