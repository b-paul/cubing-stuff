// EO solver
use lib::cube333::coordcube::EOCoord;
use lib::cube333::coordinate::Coordinate;
use lib::cube333::moves::Move;
use lib::cube333::CubieCube;

pub struct EOSolver {
    original_state: EOCoord,

    current_length: usize,
    current_sequence: Vec<Move>,

    eos_checked: usize,
    max_eos: usize,
}

impl EOSolver {
    pub fn new(cube: &CubieCube, max_eos: usize) -> Self {
        EOSolver {
            original_state: EOCoord::from_cubie_cube(cube),
            current_length: 0,
            current_sequence: vec![],
            eos_checked: 0,
            max_eos,
        }
    }
}

impl Iterator for EOSolver {
    type Item = Vec<Move>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eos_checked > self.max_eos && self.max_eos != 0 {
            return None;
        }

        while !self.original_state.apply_sequence(self.next_sequence()?).solved() {}

        self.eos_checked += 1;

        Some(self.current_sequence)
    }
}
