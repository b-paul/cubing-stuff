use super::CubieCube;

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    R,
    L,
    U,
    D,
    F,
    B,
}

#[derive(Copy, Clone)]
pub struct Move {
    ty: MoveType,
    count: u8,
}

// I don't want to have the default derive debug for this!
impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.count {
            1 => write!(f, "{:?}", self.ty),
            3 => write!(f, "{:?}'", self.ty),
            _ => write!(f, "{:?}{}", self.ty, self.count),
        }
    }
}

pub trait MoveGenerator {
    /// The amount of moves that are available in the moveset.
    const SIZE: usize;
    /// A list of all valid moves. The index of a move in this list will be the same index used
    /// when accessing the move table.
    const MOVE_LIST: &'static [Move];
}

impl From<Move> for usize {
    fn from(mv: Move) -> usize {
        (mv.count as usize - 1) * 6 + mv.ty as usize
    }
}

macro_rules! mv {
    ($ty:ident, $count: expr) => {
        Move {
            ty: MoveType::$ty,
            count: $count,
        }
    };
}

pub struct Htm;

impl MoveGenerator for Htm {
    const SIZE: usize = 18;
    const MOVE_LIST: &'static [Move] = &[
        mv!(R, 1),
        mv!(L, 1),
        mv!(U, 1),
        mv!(D, 1),
        mv!(F, 1),
        mv!(B, 1),
        mv!(R, 2),
        mv!(L, 2),
        mv!(U, 2),
        mv!(D, 2),
        mv!(F, 2),
        mv!(B, 2),
        mv!(R, 3),
        mv!(L, 3),
        mv!(U, 3),
        mv!(D, 3),
        mv!(F, 3),
        mv!(B, 3),
    ];
}

impl From<MoveType> for usize {
    fn from(mv: MoveType) -> Self {
        match mv {
            MoveType::R => 0,
            MoveType::L => 1,
            MoveType::U => 2,
            MoveType::D => 3,
            MoveType::F => 4,
            MoveType::B => 5,
        }
    }
}

const CO_OFFSETS: [[u8; 8]; 6] = [
    [2, 0, 0, 1, 1, 0, 0, 2],
    [0, 1, 2, 0, 0, 2, 1, 0],
    [0; 8],
    [0; 8],
    [1, 2, 0, 0, 2, 1, 0, 0],
    [0, 0, 1, 2, 0, 0, 2, 1],
];
const CP_OFFSETS: [[usize; 8]; 6] = [
    [4, 1, 2, 0, 7, 5, 6, 3],
    [0, 5, 1, 3, 4, 6, 2, 7],
    [3, 0, 1, 2, 4, 5, 6, 7],
    [0, 1, 2, 3, 5, 6, 7, 4],
    [1, 5, 2, 3, 0, 4, 6, 7],
    [0, 1, 6, 2, 4, 5, 7, 3],
];
const EO_OFFSETS: [[u8; 12]; 6] = [
    [0; 12],
    [0; 12],
    [0; 12],
    [0; 12],
    [1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
];
const EP_OFFSETS: [[usize; 12]; 6] = [
    [0, 1, 2, 4, 11, 5, 6, 3, 8, 9, 10, 7],
    [0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
    [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    [5, 1, 2, 3, 0, 8, 6, 7, 4, 9, 10, 11],
    [0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11],
];

impl CubieCube {
    // This function doesn't really need to be fast since coordinates exist
    pub fn make_move(&self, mv: Move) -> CubieCube {
        let mut r = self.clone();
        for _ in 0..mv.count {
            r = r.make_move_type(mv.ty);
        }
        r
    }

    fn make_move_type(&self, mv: MoveType) -> CubieCube {
        let co_offsets = CO_OFFSETS[mv as usize];
        let cp_offsets = CP_OFFSETS[mv as usize];
        let eo_offsets = EO_OFFSETS[mv as usize];
        let ep_offsets = EP_OFFSETS[mv as usize];

        let mut co = [0; 8];
        let mut cp = [0; 8];
        let mut eo = [0; 12];
        let mut ep = [0; 12];

        for i in 0..8 {
            co[i] = (self.co[cp_offsets[i]] + co_offsets[i]) % 3;
            cp[i] = self.cp[cp_offsets[i]];
        }

        for i in 0..12 {
            eo[i] = (self.eo[ep_offsets[i]] + eo_offsets[i]) % 2;
            ep[i] = self.ep[ep_offsets[i]];
        }

        CubieCube { co, cp, eo, ep }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube333::coordcube::*;
    #[test]
    fn r_loop() {
        let mut cube = CubieCube {
            co: [0; 8],
            cp: [0; 8],
            eo: [0; 12],
            ep: [0; 12],
        };
        for _ in 0..4 {
            cube = cube.make_move(Move {
                ty: MoveType::R,
                count: 1,
            });
        }
        assert_eq!(
            cube,
            CubieCube {
                co: [0; 8],
                cp: [0; 8],
                eo: [0; 12],
                ep: [0; 12],
            }
        );
    }
}
