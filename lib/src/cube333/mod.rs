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

impl From<CubieCube> for StickerCube {
    fn from(cubie_cube: CubieCube) -> Self {
        let mut sticker_cube = StickerCube::SOLVED;

        for (piece, pos) in cubie_cube.ep.zip(cubie_cube.eo).zip(Edge::ARRAY) {
            sticker_cube.place_edge(piece.into(), pos.into());
        }
        for (piece, pos) in cubie_cube.cp.zip(cubie_cube.co).zip(Corner::ARRAY) {
            sticker_cube.place_corner(piece.into(), pos.into());
        }

        sticker_cube
    }
}

impl TryFrom<StickerCube> for CubieCube {
    type Error = ();

    fn try_from(sticker_cube: StickerCube) -> Result<Self, Self::Error> {
        let mut co = [CornerTwist::Oriented; 8];
        let mut cp = [Corner::UBL; 8];
        let mut eo = [EdgeFlip::Oriented; 12];
        let mut ep = [Edge::UB; 12];

        for (i, c) in Corner::ARRAY.into_iter().enumerate() {
            let corner = sticker_cube.corner_at(c.into())?;
            co[i] = corner.ud_orientation();
            cp[i] = corner.piece();
        }

        for (i, e) in Edge::ARRAY.into_iter().enumerate() {
            let edge = sticker_cube.edge_at(e.into())?;
            eo[i] = edge.fb_orientation();
            ep[i] = edge.piece();
        }

        Ok(CubieCube { co, cp, eo, ep })
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

    pub fn face_to_sticker(&self, face: Face) -> Sticker {
        self.centers[face as usize]
    }

    pub fn edge_at(&self, pos: EdgePos) -> Result<EdgePos, ()> {
        let s1 = self[pos];
        let s2 = self[pos.flip()];
        let f1 = self.sticker_to_face(s1);
        let f2 = self.sticker_to_face(s2);
        (f1, f2).try_into()
    }

    pub fn corner_at(&self, pos: CornerPos) -> Result<CornerPos, ()> {
        let s1 = self[pos];
        let s2 = self[pos.clockwise()];
        let s3 = self[pos.anticlockwise()];
        let f1 = self.sticker_to_face(s1);
        let f2 = self.sticker_to_face(s2);
        let f3 = self.sticker_to_face(s3);
        (f1, f2, f3).try_into()
    }

    pub fn place_edge(&mut self, piece: EdgePos, pos: EdgePos) {
        self[pos] = self.face_to_sticker(piece.into());
        self[pos.flip()] = self.face_to_sticker(piece.flip().into());
    }

    pub fn place_corner(&mut self, piece: CornerPos, pos: CornerPos) {
        self[pos] = self.face_to_sticker(piece.into());
        self[pos.clockwise()] = self.face_to_sticker(piece.clockwise().into());
        self[pos.anticlockwise()] = self.face_to_sticker(piece.anticlockwise().into());
    }
}

use std::ops::{Index, IndexMut};

impl Index<EdgePos> for StickerCube {
    type Output = Sticker;

    fn index(&self, index: EdgePos) -> &Self::Output {
        &self.edges[index as usize]
    }
}

impl IndexMut<EdgePos> for StickerCube {
    fn index_mut(&mut self, index: EdgePos) -> &mut Self::Output {
        &mut self.edges[index as usize]
    }
}

impl Index<CornerPos> for StickerCube {
    type Output = Sticker;

    fn index(&self, index: CornerPos) -> &Self::Output {
        &self.corners[index as usize]
    }
}

impl IndexMut<CornerPos> for StickerCube {
    fn index_mut(&mut self, index: CornerPos) -> &mut Self::Output {
        &mut self.corners[index as usize]
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
        let (f1, f2) = self.into();
        (f2, f1).try_into().unwrap()
    }

    pub fn piece(self) -> Edge {
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

impl CornerPos {
    pub fn clockwise(self) -> CornerPos {
        let (f1, f2, f3) = self.into();
        (f3, f1, f2).try_into().unwrap()
    }

    pub fn anticlockwise(self) -> CornerPos {
        let (f1, f2, f3) = self.into();
        (f2, f3, f1).try_into().unwrap()
    }

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

    // Orientation with U and D on top and bottom
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

impl From<Face> for usize {
    fn from(value: Face) -> Self {
        match value {
            Face::U => 0,
            Face::L => 1,
            Face::F => 2,
            Face::R => 3,
            Face::B => 4,
            Face::D => 5,
        }
    }
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
