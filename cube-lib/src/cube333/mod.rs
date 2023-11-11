// Cube333 module

use crate::TryFromIntToEnumError;
use thiserror::Error;

/// Implementation of a cube based on coordinates, which are more performant than arrays when
/// making moves but harder to work with.
pub mod coordcube;
/// Definitions for corner pieces
pub mod corner;
/// Definitions for edge pieces
pub mod edge;
/// Defines move types and implements application of moves to the CubieCube.
pub mod moves;

use corner::{Corner, CornerPos, CornerTwist};
use edge::{Edge, EdgeFlip, EdgePos};

/// Errors for when converting a stickered piece to a piece type
#[derive(Debug, Error)]
pub enum StickerToPieceError {
    // i dont like this error message :(
    /// The stickers of a piece did not correspond to a legal piece.
    #[error("a combination of stickers did not give a valid piece")]
    InvalidPiece,
}

/// An implementation of a Rubik's cube which represents itself using pieces in an array. A Piece
/// has an orientation and a permutation to uniquely identify itself. Note that there exists some
/// CubieCubes which are not solvable (e.g. a corner twist).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CubieCube {
    // May want to make these values not public
    /// Corner orientation
    pub co: [CornerTwist; 8],
    /// Corner permutation
    pub cp: [Corner; 8],
    /// Edge orientation
    pub eo: [EdgeFlip; 12],
    /// Edge permutation
    pub ep: [Edge; 12],
}

impl CubieCube {
    /// The solved cube stored as a const.
    pub const SOLVED: CubieCube = CubieCube {
        co: [CornerTwist::Oriented; 8],
        cp: Corner::ARRAY,
        eo: [EdgeFlip::Oriented; 12],
        ep: Edge::ARRAY,
    };
}

impl From<CubieCube> for StickerCube {
    fn from(cubie_cube: CubieCube) -> Self {
        let mut sticker_cube = StickerCube::SOLVED;

        for ((piece, orientation), pos) in cubie_cube
            .ep
            .into_iter()
            .zip(cubie_cube.eo)
            .zip(Edge::ARRAY)
        {
            sticker_cube.place_edge((piece, orientation).into(), pos.into());
        }
        for ((piece, orientation), pos) in cubie_cube
            .cp
            .into_iter()
            .zip(cubie_cube.co)
            .zip(Corner::ARRAY)
        {
            sticker_cube.place_corner((piece, orientation).into(), pos.into());
        }

        sticker_cube
    }
}

/// An error type for conversion from a sticker representation to cubie representation.
#[derive(Debug, Error)]
pub enum StickerToCubieError {
    /// A corner had stickers corresponding to an illegal state
    #[error("The {0} piece has invalid stickers")]
    BadCorner(Corner),

    /// An edge had stickers corresponding to an illegal state
    #[error("The {0} piece has invalid stickers")]
    BadEdge(Edge),
}

impl TryFrom<StickerCube> for CubieCube {
    type Error = StickerToCubieError;

    fn try_from(sticker_cube: StickerCube) -> Result<Self, Self::Error> {
        let mut co = [CornerTwist::Oriented; 8];
        let mut cp = [Corner::UBL; 8];
        let mut eo = [EdgeFlip::Oriented; 12];
        let mut ep = [Edge::UB; 12];

        for (i, c) in Corner::ARRAY.into_iter().enumerate() {
            let corner = sticker_cube
                .corner_at(c.into())
                .map_err(|_| StickerToCubieError::BadCorner(c))?;
            co[i] = corner.ud_orientation();
            cp[i] = corner.piece();
        }

        for (i, e) in Edge::ARRAY.into_iter().enumerate() {
            let edge = sticker_cube
                .edge_at(e.into())
                .map_err(|_| StickerToCubieError::BadEdge(e))?;
            eo[i] = edge.fb_orientation();
            ep[i] = edge.piece();
        }

        Ok(CubieCube { co, cp, eo, ep })
    }
}

/// A Rubik's cube implementation that represents itself using stickers. Faces go in the order of
/// UP LEFT FRONT RIGHT BACK DOWN. Stickers go left to right, top to bottom, starting at the top
/// right.
pub struct StickerCube {
    pub(crate) edges: [Sticker; 24],
    pub(crate) corners: [Sticker; 24],
    pub(crate) centers: [Sticker; 6],
}

use Sticker as S;

impl StickerCube {
    /// The solved cube stored as a const.
    pub const SOLVED: StickerCube = StickerCube {
        edges: [
            S::S1,
            S::S1,
            S::S1,
            S::S1,
            S::S2,
            S::S2,
            S::S2,
            S::S2,
            S::S3,
            S::S3,
            S::S3,
            S::S3,
            S::S4,
            S::S4,
            S::S4,
            S::S4,
            S::S5,
            S::S5,
            S::S5,
            S::S5,
            S::S6,
            S::S6,
            S::S6,
            S::S6,
        ],
        corners: [
            S::S1,
            S::S1,
            S::S1,
            S::S1,
            S::S2,
            S::S2,
            S::S2,
            S::S2,
            S::S3,
            S::S3,
            S::S3,
            S::S3,
            S::S4,
            S::S4,
            S::S4,
            S::S4,
            S::S5,
            S::S5,
            S::S5,
            S::S5,
            S::S6,
            S::S6,
            S::S6,
            S::S6,
        ],
        centers: [S::S1, S::S2, S::S3, S::S4, S::S5, S::S6],
    };

    pub(crate) fn sticker_to_face(&self, sticker: Sticker) -> Face {
        self.centers
            .iter()
            .position(|c| *c == sticker)
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub(crate) fn face_to_sticker(&self, face: Face) -> Sticker {
        self.centers[face as usize]
    }

    pub(crate) fn edge_at(&self, pos: EdgePos) -> Result<EdgePos, StickerToPieceError> {
        let s1 = self[pos];
        let s2 = self[pos.flip()];
        let f1 = self.sticker_to_face(s1);
        let f2 = self.sticker_to_face(s2);
        (f1, f2).try_into()
    }

    pub(crate) fn corner_at(&self, pos: CornerPos) -> Result<CornerPos, StickerToPieceError> {
        let s1 = self[pos];
        let s2 = self[pos.clockwise()];
        let s3 = self[pos.anticlockwise()];
        let f1 = self.sticker_to_face(s1);
        let f2 = self.sticker_to_face(s2);
        let f3 = self.sticker_to_face(s3);
        (f1, f2, f3).try_into()
    }

    pub(crate) fn place_edge(&mut self, piece: EdgePos, pos: EdgePos) {
        self[pos] = self.face_to_sticker(piece.into());
        self[pos.flip()] = self.face_to_sticker(piece.flip().into());
    }

    pub(crate) fn place_corner(&mut self, piece: CornerPos, pos: CornerPos) {
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

/// Represents a sticker relative to the original starting orientation. Using the standard Rubik's
/// cube colour scheme with white on top and green on front, these stickers would represent:
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sticker {
    /// White
    S1,
    /// Orange
    S2,
    /// Green
    S3,
    /// Red
    S4,
    /// Blue
    S5,
    /// Yellow
    S6,
}

/// Enum to represent faces on a cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Face {
    /// Up face
    U,
    /// Left face
    L,
    /// Front face
    F,
    /// Right face
    R,
    /// Back face
    B,
    /// Down face
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
    type Error = TryFromIntToEnumError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Face::U),
            1 => Ok(Face::L),
            2 => Ok(Face::F),
            3 => Ok(Face::R),
            4 => Ok(Face::B),
            5 => Ok(Face::D),
            _ => Err(TryFromIntToEnumError::OutOfBounds),
        }
    }
}

impl std::fmt::Display for StickerCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn str(s: Sticker) -> &'static str {
            match s {
                S::S1 => "u ",
                S::S2 => "l ",
                S::S3 => "f ",
                S::S4 => "r ",
                S::S5 => "b ",
                S::S6 => "d ",
            }
        }

        use CornerPos::*;
        use EdgePos::*;

        write!(
            f,
            "
      {}{}{}
      {}{}{}
      {}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
      {}{}{}
      {}{}{}
      {}{}{}
",
            str(self[UBL]),
            str(self[UB]),
            str(self[UBR]),
            str(self[UL]),
            str(self.centers[0]),
            str(self[UR]),
            str(self[UFL]),
            str(self[UF]),
            str(self[UFR]),
            str(self[LUB]),
            str(self[LU]),
            str(self[LUF]),
            str(self[FUL]),
            str(self[FU]),
            str(self[FUR]),
            str(self[RUF]),
            str(self[RU]),
            str(self[RUB]),
            str(self[BUL]),
            str(self[BU]),
            str(self[BUR]),
            str(self[LB]),
            str(self.centers[1]),
            str(self[LF]),
            str(self[FL]),
            str(self.centers[2]),
            str(self[FR]),
            str(self[RF]),
            str(self.centers[3]),
            str(self[RB]),
            str(self[BL]),
            str(self.centers[4]),
            str(self[BR]),
            str(self[LDB]),
            str(self[LD]),
            str(self[LDF]),
            str(self[FDL]),
            str(self[FD]),
            str(self[FDR]),
            str(self[RDF]),
            str(self[RD]),
            str(self[RDB]),
            str(self[BDL]),
            str(self[BD]),
            str(self[BDR]),
            str(self[DFL]),
            str(self[DF]),
            str(self[DFR]),
            str(self[DL]),
            str(self.centers[5]),
            str(self[DR]),
            str(self[DBL]),
            str(self[DB]),
            str(self[DBR]),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pieces_on_solved_cube() {
        use super::StickerCube;
        use super::edge::EdgePos::*;
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
