/*
use std::collections::HashSet;

/// Puzzles that implement this trait are groups. There isn't really a way to make the compiler
/// verify that this is true so the user has to! If this trait is implemented for a puzzle that is
/// not a group, then everything will be broken!
pub trait Group {
    // Maybe this is required ?!??!? idk
    //fn operation(a: Self, b: Self) -> Self;
}

/// Move trait (do i need this? what does it implement? is it just a marker trait?)
pub trait Move {}

/// A trait for groups which specifically are groups.
pub trait GroupPuzzle: Group + Sized + Eq + std::hash::Hash {
    type Move: Sized;

    const GENERATOR_SIZE: usize;

    /// List of legal moves.
    const GENERATOR: [Self::Move; Self::GENERATOR_SIZE];

    /// The default solved state.
    const SOLVED_STATE: Self;

    /// Apply a move M to the puzzle.
    fn apply_move(&self, mv: Self::Move) -> Self;

    /// Mutably apply a move M to the puzzle.
    fn apply_move_mut(&mut self, mv: Self::Move) {
        *self = self.apply_move(mv);
    }
}

/// A coordinate of a puzzle group P is a group (S, <*>) with some surjective homomorphism h(x)
/// and <*> is an arbitrary operation. Note that by Lagrange's theorem |S| divides |P|.
///
/// Coordinates are useful for high performance implementations of a puzzle since we can make a
/// lookup table to implement the group operation, and lookup tables are fast.
///
/// The coordinate must implement `Into<usize>` to allow for indexing into these lookup tables.
pub trait Coordinate<P: GroupPuzzle + Clone>: Into<usize> + Copy {
    /// The order of the coordinate group.
    const ORDER: usize;

    /// Function that converts a puzzle to a coordinate value. It must be a surjective homomorphism
    /// to z_n, where n is a factor of the order of the puzzle group.
    fn from_puzzle(puzzle: P) -> Self;

    // should return an err type
    // has 1 million clones :(
    /// Generates a table containing move+state mappings to states after applying a move to a
    /// coordinate.
    fn gen_move_table() -> Result<[[Self; Self::ORDER]; P::GENERATOR_SIZE], ()> {
        let mut relevant_states: HashSet<P> = HashSet::new();
        let mut stack: Vec<P> = Vec::new();
        let mut opt_table = [[None; Self::ORDER]; P::GENERATOR_SIZE];

        relevant_states.insert(P::SOLVED_STATE);
        stack.push(P::SOLVED_STATE);

        while let Some(p) = stack.pop() {
            for (i, mv) in P::GENERATOR.into_iter().enumerate() {
                let new_p = p.clone().apply_move(*mv);
                if relevant_states.contains(&new_p) {
                    continue;
                }
                relevant_states.insert(new_p.clone());
                stack.push(new_p.clone());
                opt_table[i][Self::from_puzzle(p.clone()).into()] =
                    Some(Self::from_puzzle(new_p.clone()));
            }
        }

        let table = opt_table.try_map(|xs| xs.try_map(|x| x)).ok_or(());

        table
    }

    /// Applies a move to some coordinate using a move table.
    fn apply_move(self, table: &[Self; P::GENERATOR_SIZE]) -> Self {
        table[self.into()]
    }
}
*/
