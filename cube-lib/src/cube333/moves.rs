use super::CubieCube;

/// Represents each type of move. Note that the `Move` struct uses this variable along with a
/// counter to represents move such as R2 or U'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Move333Type {
    /// Right
    R,
    /// Left
    L,
    /// Up
    U,
    /// Down
    D,
    /// Front
    F,
    /// Back
    B,
}

/// Stores a move type and counter. An anti-clockwise move will have a count of 3.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub struct Move333 {
    pub ty: Move333Type,
    pub count: u8,
}

impl crate::moves::Move for Move333 {
    fn inverse(self) -> Self {
        Self {
            ty: self.ty,
            count: (4 - self.count).rem_euclid(4),
        }
    }

    fn commutes_with(&self, b: &Self) -> bool {
        use Move333Type as MT;
        match self.ty {
            MT::R | MT::L => [MT::R, MT::L].contains(&b.ty),
            MT::U | MT::D => [MT::U, MT::D].contains(&b.ty),
            MT::F | MT::B => [MT::F, MT::B].contains(&b.ty),
        }
    }

    /* TODO
    fn cancel(self, b: Self) -> [Option<Self>; 2] {
        if self.ty == b.ty {
            let mv = Move333 {
                ty: self.ty,
                count: (self.count + b.count) % 4,
            };
            let mv = (mv.count != 0).then_some(mv);
            [
                mv,
                None
            ]
        } else {
            [Some(self), Some(b)]
        }
    }
    */
}

// I don't want to have the default derive debug for this!
impl std::fmt::Debug for Move333 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.count {
            1 => write!(f, "{:?}", self.ty),
            3 => write!(f, "{:?}'", self.ty),
            _ => write!(f, "{:?}{}", self.ty, self.count),
        }
    }
}

/// A trait to classify a type as a move generator. A move generator is a set which can be used to
/// generate a set, i.e. find every combination of moves using moves in the move generator to find
/// unique states.
pub trait MoveGenerator {
    /// The amount of moves that are available in the moveset.
    const SIZE: usize;
    /// A list of all valid moves. The index of a move in this list will be the same index used
    /// when accessing the move table.
    const MOVE_LIST: &'static [Move333];
}

impl From<Move333> for usize {
    fn from(mv: Move333) -> usize {
        (mv.count as usize - 1) * 6 + mv.ty as usize
    }
}

/// Create a move by specifying a move type and move count. Note that you do not need to specify
/// for example Move333Type::R, you only need to specify R. (TODO this might not be a good idea).
#[macro_export]
macro_rules! mv {
    ($ty:ident, $count: expr) => {
        Move333 {
            ty: Move333Type::$ty,
            count: $count,
        }
    };
}

/// Type for Half Turn Metric
pub struct Htm;

impl MoveGenerator for Htm {
    const SIZE: usize = 18;
    const MOVE_LIST: &'static [Move333] = &[
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

impl From<Move333Type> for usize {
    fn from(mv: Move333Type) -> Self {
        match mv {
            Move333Type::R => 0,
            Move333Type::L => 1,
            Move333Type::U => 2,
            Move333Type::D => 3,
            Move333Type::F => 4,
            Move333Type::B => 5,
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
    [0, 2, 6, 3, 4, 1, 5, 7],
    [3, 0, 1, 2, 4, 5, 6, 7],
    [0, 1, 2, 3, 5, 6, 7, 4],
    [1, 5, 2, 3, 0, 4, 6, 7],
    [0, 1, 3, 7, 4, 5, 2, 6],
];
const EO_OFFSETS: [[u8; 12]; 6] = [
    [0; 12],
    [0; 12],
    [0; 12],
    [0; 12],
    [1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1],
];
const EP_OFFSETS: [[usize; 12]; 6] = [
    [0, 1, 2, 8, 4, 5, 6, 11, 7, 9, 10, 3],
    [0, 10, 2, 3, 4, 9, 6, 7, 8, 1, 5, 11],
    [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11],
    [9, 1, 2, 3, 8, 5, 6, 7, 0, 4, 10, 11],
    [0, 1, 11, 3, 4, 5, 10, 7, 8, 9, 2, 6],
];

impl CubieCube {
    // This function doesn't really need to be fast since coordinates exist
    /// Copy the cube, apply a move to it, then return the new cube.
    pub fn make_move(&self, mv: Move333) -> CubieCube {
        let mut r = self.clone();
        for _ in 0..mv.count {
            r = r.make_move_type(mv.ty);
        }
        r
    }

    fn make_move_type(&self, mv: Move333Type) -> CubieCube {
        let co_offsets = CO_OFFSETS[mv as usize];
        let cp_offsets = CP_OFFSETS[mv as usize];
        let eo_offsets = EO_OFFSETS[mv as usize];
        let ep_offsets = EP_OFFSETS[mv as usize];

        let selfco: [u8; 8] = self.co.map(|t| t.into());
        let selfcp: [u8; 8] = self.cp.map(|t| t.into());
        let selfeo: [u8; 12] = self.eo.map(|t| t.into());
        let selfep: [u8; 12] = self.ep.map(|t| t.into());

        let mut co = [0; 8];
        let mut cp = [0; 8];
        let mut eo = [0; 12];
        let mut ep = [0; 12];

        for i in 0..8 {
            co[i] = (selfco[cp_offsets[i]] + co_offsets[i]) % 3;
            cp[i] = selfcp[cp_offsets[i]];
        }

        for i in 0..12 {
            eo[i] = (selfeo[ep_offsets[i]] + eo_offsets[i]) % 2;
            ep[i] = selfep[ep_offsets[i]];
        }

        let co = co.map(|n| n.try_into().unwrap());
        let cp = cp.map(|n| n.try_into().unwrap());
        let eo = eo.map(|n| n.try_into().unwrap());
        let ep = ep.map(|n| n.try_into().unwrap());

        CubieCube { co, cp, eo, ep }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn r_loop() {
        let mut cube = CubieCube::SOLVED;
        for _ in 0..4 {
            cube = cube.make_move(Move333 {
                ty: Move333Type::B,
                count: 1,
            });
        }
        assert_eq!(cube, CubieCube::SOLVED);
    }
}
