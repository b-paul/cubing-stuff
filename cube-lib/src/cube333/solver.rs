// Solver

use crate::cube333::coordcube::{COCoord, EOCoord};
use crate::cube333::moves::Move;
use crate::cube333::CubieCube;

pub struct Solver {}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct ESliceCoord(u16);

impl From<ESliceCoord> for usize {
    fn from(coord: ESliceCoord) -> usize {
        coord.0 as usize
    }
}

struct P1Cube {
    _eo: EOCoord,
    _co: COCoord,
    _e_slice: ESliceCoord,
}

impl P1Cube {
    fn from_cubie_cube(_cube: CubieCube) -> Self {
        todo!()
    }

    fn solved(&self) -> bool {
        todo!()
    }

    // Apply move
}

impl Solver {
    pub fn solve_cubie(&self, cube: CubieCube) -> Vec<Move> {
        let p1 = P1Cube::from_cubie_cube(cube);

        if p1.solved() {
            todo!()
        }

        todo!()
    }
}
