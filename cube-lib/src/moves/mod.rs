//! Module for puzzle move generics and related functionality

/// Type of move that can be made to a puzzle.
pub trait Move {
    /// Invert a move.
    ///
    /// If `X` is a moves and `X^{-1}` is its inverse and `o` is composition, then
    /// `X o X^{-1} = X^{-1} o X = e` where `e` is a move which does nothing.
    fn inverse(self) -> Self
    where
        Self: Sized;

    /// Returns whether the two moves commute, i.e. can be swapped when adjacent.
    ///
    /// If A and B are moves, then `A.commutes(B)` iff
    /// `A B = B A`
    fn commutes_with(&self, b: &Self) -> bool;

    /* TODO
    /// Returns the cancelled list of the moves `self` and `b`, with None signifying no move.
    ///
    /// - It is assumed that if a and b don't cancel, then `a.cancel(b) = [Some(a), Some(b)]`.
    /// - It is assumed that commutativity and non-commutativity are preserved under cancallation.
    /// - It is assumed that `a.cancel(a)` always cancels.
    /// - It is also assumed that `a.cancel(b) == b.cancel(a)` if and only if a and b cancel.
    ///
    /// ```rust
    /// # use cube_lib::mv;
    /// # use cube_lib::cube333::moves::{Move333, Move333Type};
    /// # use cube_lib::moves::Move;
    /// # fn main() {
    /// assert_eq!(mv!(R, 1).cancel(mv!(U, 3)), [Some(mv!(R, 1)), Some(mv!(U, 3))]);
    /// assert_eq!(mv!(R, 1).cancel(mv!(R, 1)), [Some(mv!(R, 2)), None]);
    /// assert_eq!(mv!(R, 1).cancel(mv!(R, 3)), [None, None]);
    /// # }
    /// ```
    fn cancel(self, b: Self) -> [Option<Self>; 2]
    where
        Self: Sized;
    */
}

/// A sequence of moves (also known as an algorithm) for some specific type of move.
pub struct MoveSequence<M: Move>(Vec<M>);

impl<M: Move> MoveSequence<M> {
    /// Invert a sequence of moves.
    ///
    /// If `X` is a sequence of moves and `X^{-1}` is its inverse and `o` is composition, then
    /// `X o X^{-1} = X^{-1} o X = e` where `e` is the empty sequence.
    pub fn invert(self) -> Self {
        Self(self.0.into_iter().rev().map(|m| m.inverse()).collect())
    }

    /* TODO
    /// Cancel an alg completely, including rearrangement of commutative moves (I'm so scared of
    /// writing this aaahhhh)
    pub fn cancel(self) -> Self {
        Self(self.0.group_by(|a, b| a.commutes_with(b)).flat_map(|moves| {
            // This is an extra vec that isn't needed! We could just work on a singular global vec
            // but yuck
            let mut cancelled_moves = Vec::new();

            for mv in moves {
                for mv2 in cancelled_moves.iter_mut() {
                    /*
                    if let Some(mv) = mv.cancel(mv2) {
                        mv2 = mv;
                        break;
                    }
                    */
                }
            }

            cancelled_moves
        }).collect())
    }
    */
}
