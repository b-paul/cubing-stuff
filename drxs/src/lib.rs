use cube_lib::{
    cube333::{
        CubieCube,
        axis::Axis,
        corner::CornerTwist,
        edge::EdgeFlip,
        moves::{Move333, Move333Type},
    },
    moves::{Move, MoveSequence},
    mv,
};

pub fn next_ty(t: Move333Type) -> Option<Move333Type> {
    match t {
        Move333Type::R => Some(Move333Type::L),
        Move333Type::L => Some(Move333Type::U),
        Move333Type::U => Some(Move333Type::D),
        Move333Type::D => Some(Move333Type::F),
        Move333Type::F => Some(Move333Type::B),
        Move333Type::B => None,
    }
}

/// Determines whether a CubieCube is in dr-xs for some axis.
fn is_solved(c: &CubieCube) -> bool {
    use CornerTwist as CT;
    #[rustfmt::skip]
    const COS: [[CornerTwist; 8]; 3] = [[CT::Oriented; 8],
    [CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise],
    [CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise, CT::AntiClockwise, CT::Clockwise]];
    const IDXS: [[usize; 4]; 3] = [[8, 9, 10, 11], [0, 2, 4, 6], [1, 3, 5, 7]];

    fn idx_i(a: Axis, j: usize) -> usize {
        match a {
            Axis::FB => (j + 2) % 3,
            Axis::LR => (j + 1) % 3,
            Axis::UD => j,
        }
    }

    Axis::AXES.iter().any(|&a| {
        let co = c.axis_co(a);
        let eo = c.axis_eo(a);
        (0..3).any(|j| {
            let idxs = IDXS[idx_i(a, j)];
            COS[j] == co
                && idxs.into_iter().all(|k| match a {
                    Axis::FB => c.ep[k].s_slice(),
                    Axis::LR => c.ep[k].m_slice(),
                    Axis::UD => c.ep[k].e_slice(),
                })
                && (idxs.into_iter().all(|k| eo[k] == EdgeFlip::Oriented)
                    || idxs.into_iter().all(|k| eo[k] == EdgeFlip::Flipped))
        })
    })
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

    fn add_move(&mut self, mv: Move333) {
        self.moves.push(mv);
        self.cube = self.cube.clone().make_move(mv);
    }

    fn pop_move(&mut self) -> Option<Move333> {
        let m = self.moves.pop()?;
        self.cube = self.cube.clone().make_move(m.inverse());
        Some(m)
    }

    fn choose_new_move(&mut self) {
        assert!(self.depth > self.moves.len());
        use Move333Type as T;

        match self.moves.last().copied() {
            // Don't dupe R moves
            Some(m) if m.ty == T::R => self.add_move(mv!(L, 1)),
            // We never check L R because we'll do R L instead
            // hence we skip straight to L U
            Some(m) if m.ty == T::L => self.add_move(mv!(U, 1)),
            _ => self.add_move(mv!(R, 1)),
        }
    }

    /// Increase the current search depth and reset the search space.
    fn increase_depth(&mut self) {
        assert!(self.moves.is_empty());

        self.depth += 1;

        for _ in 0..self.depth {
            self.choose_new_move();
        }
    }

    /// Attempts to get the next move in the search space. If we have exhausted the search space,
    /// false is returned.
    fn next_state(&mut self) -> bool {
        let Some(last_end) = self.pop_move() else {
            return false;
        };

        if last_end.count < 3 {
            assert!(last_end.count >= 1);
            let m = Move333 {
                count: last_end.count + 1,
                ..last_end
            };
            self.add_move(m);
            true
        } else {
            assert!(last_end.count == 3);

            // i love match match
            match match (
                next_ty(last_end.ty),
                self.moves.last().cloned().map(|m| m.ty),
            ) {
                (Some(ty), Some(ty2)) if ty == ty2.opposite() => next_ty(ty).and_then(next_ty),
                (Some(ty), Some(ty2)) if ty == ty2 => next_ty(ty),
                (Some(ty), _) => Some(ty),
                (None, _) => None,
            } {
                Some(ty) => {
                    let m = Move333 { ty, count: 1 };
                    self.add_move(m);
                    true
                }
                None => {
                    if self.next_state() {
                        self.choose_new_move();
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

impl Iterator for LinearSolver {
    type Item = MoveSequence<Move333>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.depth <= self.max_depth {
            // cursed do while loop
            while {
                if is_solved(&self.cube) {
                    let mvs = self.moves.clone();
                    // skip over the variations of R R' L L' moves
                    for _ in 0..5 {
                        self.next_state();
                    }
                    if !self.next_state() {
                        self.increase_depth();
                    }
                    return Some(MoveSequence(mvs));
                }
                self.next_state()
            } {}
            self.increase_depth();
        }
        None
    }
}
