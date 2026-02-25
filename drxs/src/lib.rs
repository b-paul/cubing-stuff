use cube_lib::{
    cube333::{CubieCube, axis::Axis, corner::CornerTwist, moves::Move333},
    moves::MoveSequence,
};

/// Determines whether a CubieCube is in dr-xs for some axis.
fn is_solved(c: &CubieCube) -> bool {
    use CornerTwist as CT;
    #[rustfmt::skip]
    const COS: [[CornerTwist; 8]; 3] = [[CT::Oriented; 8], [CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise], [CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise]];

    // this is just co atm lol
    Axis::AXES.iter().any(|&a| COS.contains(&c.axis_co(a)))
}

/// Finds linear dr-xs solutions (on the normal side of a scramble) with an iterator interface.
pub struct LinearSolver {
    /// The current search depth that the solver is at. This will increase over time.
    depth: usize,
    /// The maximim permitted search depth. The solver will terminate past this depth. The user can
    /// control this maximum with `set_max_depth`.
    max_depth: usize,
    cube: CubieCube,
    moves: Vec<Move333>,
}

impl LinearSolver {
    pub fn new(cube: CubieCube) -> LinearSolver {
        LinearSolver {
            depth: 0,
            max_depth: 20,
            cube,
            moves: vec![],
        }
    }

    // TODO this stuff should really be in a builder, not on the (potentially currently iterating)
    // solver.

    pub fn set_min_depth(&mut self, d: usize) {
        // SUPER SCARY THAT THIS IS PUBLIC oh well i'm the only one actually using this code lolol
        self.depth = d;
    }

    pub fn set_max_depth(&mut self, d: usize) {
        self.max_depth = d;
    }

    /// Increase the current search depth and reset the search space.
    fn increase_depth(&mut self) {
        todo!()
    }

    /// Attempts to get the next move in the search space. If we have exhausted the search space,
    /// false is returned.
    fn next_state(&mut self) -> bool {
        todo!()
    }
}

impl Iterator for LinearSolver {
    type Item = MoveSequence<Move333>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.depth <= self.max_depth {
            while self.next_state() {
                if is_solved(&self.cube) {
                    return Some(MoveSequence(self.moves.clone()));
                }
            }
            self.increase_depth();
        }
        None
    }
}
