//! Module for puzzle move generics and related functionality

/// Enum for representing the cancellation of two moves.
/// See [`cancel`](Move::cancel).
#[derive(Eq, PartialEq)]
pub enum Cancellation<M: Move> {
    /// The moves cancelled completely.
    ///
    /// e.g. `R R'` cancels completely
    NoMove,
    /// The moves cancelled into one move.
    ///
    /// e.g. `R R` cancels into `R2`
    OneMove(M),
    /// The moves didn't cancel
    ///
    /// e.g. `R U` stays as `R U` when cancelling
    TwoMove(M, M),
}

/// Type of move that can be made to a puzzle.
pub trait Move: Eq {
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

    /// Return the cancellation of two moves.
    ///
    /// It is assumed that group axioms hold when applying cancellations.
    ///
    /// ```rust
    /// # fn main() {
    /// use cube_lib::mv;
    /// use cube_lib::cube333::moves::{Move333, Move333Type};
    /// use cube_lib::moves::{Cancellation, Move};
    ///
    /// // In the context of a 3x3x3 Rubik's cube
    /// assert!(mv!(R, 1).cancel(mv!(U, 3)) == Cancellation::TwoMove(mv!(R, 1), mv!(U, 3)));
    /// assert!(mv!(R, 1).cancel(mv!(R, 1)) == Cancellation::OneMove(mv!(R, 2)));
    /// assert!(mv!(R, 1).cancel(mv!(R, 3)) == Cancellation::NoMove);
    /// # }
    /// ```
    fn cancel(self, b: Self) -> Cancellation<Self>
    where
        Self: Sized;
}

/// A sequence of moves (also known as an algorithm) for some specific type of move.
#[derive(Eq, PartialEq)]
pub struct MoveSequence<M: Move>(pub Vec<M>);

impl<M: Move> MoveSequence<M> {
    /// Invert a sequence of moves.
    ///
    /// If `X` is a sequence of moves and `X^{-1}` is its inverse and `o` is composition, then
    /// `X o X^{-1} = X^{-1} o X = e` where `e` is the empty sequence.
    pub fn inverse(self) -> Self {
        Self(self.0.into_iter().rev().map(|m| m.inverse()).collect())
    }

    /// Cancel an alg completely, including rearrangement of commutative moves. This guarantees
    /// optimal move count from cancellations.
    pub fn cancel(mut self) -> Self {
        let mut cancellation = Vec::new();
        let mut commuting_start = 0;

        for next_mv in self.0.drain(..) {
            let i = cancellation.len();
            if cancellation
                .last()
                .is_some_and(|m| next_mv.commutes_with(m))
            {
                cancellation.push(next_mv);
                for j in (commuting_start..i).rev() {
                    cancellation.swap(i - 1, j);
                    let next_mv = cancellation.pop().unwrap();
                    let mv = cancellation.pop().unwrap();
                    match mv.cancel(next_mv) {
                        Cancellation::NoMove => {
                            commuting_start = commuting_start.min(i - 2);
                            break;
                        }
                        Cancellation::OneMove(mv) => {
                            cancellation.push(mv);
                            break;
                        }
                        Cancellation::TwoMove(mv, next_mv) => {
                            cancellation.push(mv);
                            cancellation.push(next_mv);
                        }
                    }
                }
            } else {
                cancellation.push(next_mv);
                commuting_start = i;
            }
        }

        Self(cancellation)
    }
}
