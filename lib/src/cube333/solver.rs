// Solver

use crate::cube333::coordcube::{COCoord, EOCoord};
use crate::cube333::coordinate::Coordinate;
use crate::cube333::moves::{Htm, Move};
use crate::cube333::CubieCube;

pub struct Solver {}

// Combinatorial n choose r thingo
// r <= n
fn choose(n: u16, r: u16) -> u16 {
    let mut result = 1;
    for i in (n - r + 1)..=n {
        result *= i;
    }

    let mut div = 1;
    for i in 2..=r {
        div *= i
    }

    result / div
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct ESliceCoord(u16);

impl From<ESliceCoord> for usize {
    fn from(coord: ESliceCoord) -> usize {
        coord.0 as usize
    }
}

// TODO!!!! TEST!!!!!!!! THIS!!!!!!!!!
impl Coordinate for ESliceCoord {
    type Generator = Htm;
    const SIZE: usize = 495;

    fn from_cubie_cube(cube: &CubieCube) -> Self {
        let mut mask = 0;

        for i in 0..12 {
            let e = cube.ep[i];
            if e == 4 || e == 5 || e == 6 || e == 7 {
                mask |= 1 << i;
            }
        }

        // mask now contains ones at each E slice edge location (in a 12 bit sized mask)
        // http://kociemba.org/math/UDSliceCoord.htm

        let mut coord = 0;

        let mut ones = 0;
        let mut i = 0;
        while ones < 4 {
            let bit = mask & 1;

            if bit == 1 {
                ones += 1;
            } else {
                coord += choose(11 - i, 3 - ones);
            }

            i += 1;
            mask >>= 1;
        }

        ESliceCoord(coord)
    }
    fn to_cubie_cube(&self) -> CubieCube {
        // This doesnt need to be super fast since it's only used on startup so whatever
        let mut mask: i16 = 15;

        for _ in 0..self.0 {
            // https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation
            let t = mask | (mask - 1);
            mask = (t + 1) | (((!t & -(!t)) - 1) >> (mask.trailing_zeros() + 1));
        }

        // Mask is now set to be a bit mask for where each E slice edge is
        debug_assert_eq!(mask.count_ones(), 4);

        let mut ep = [12; 12];

        for edge in 4..8 {
            // Index of the least significant bit
            let index = mask.trailing_zeros() as usize;

            ep[index] = edge;

            // Remove the least significant bit
            mask &= mask - 1;
        }

        let mut i = 0;
        for edge in [0, 1, 2, 3, 8, 9, 10, 11] {
            if ep[i] != 12 {
                i += 1;
            }
            ep[i] = edge;
            i += 1;
        }

        CubieCube {
            co: [0; 8],
            cp: [0; 8],
            eo: [0; 12],
            ep,
        }
    }

    fn solved(&self) -> bool {
        self.0 == 0
    }
}

struct P1Cube {
    eo: EOCoord,
    co: COCoord,
    e_slice: ESliceCoord,
}

impl P1Cube {
    fn from_cubie_cube(cube: CubieCube) -> Self {
        P1Cube {
            eo: EOCoord::from_cubie_cube(&cube),
            co: COCoord::from_cubie_cube(&cube),
            e_slice: ESliceCoord::from_cubie_cube(&cube),
        }
    }

    fn solved(&self) -> bool {
        self.eo.solved() && self.co.solved() && self.e_slice.solved()
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
