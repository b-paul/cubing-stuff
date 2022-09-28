use super::moves::MoveGenerator;
use super::CubieCube;

/// A coordinate defines a property or properties about a cube as a number. This allows for fast
/// move application functions when trying to solve for a particular property (e.g. domino
/// reduction, corner permutation etc).
pub trait Coordinate: Into<usize> + Default + Sized + Copy + Clone {
    type Generator: MoveGenerator;
    const SIZE: usize;
    //type MoveTable: MoveTable<Self>;
    /// Take in a [`CubieCube`] an generate a coordinate which represents the state of a property
    /// for the cube.
    fn from_cubie_cube(cube: &CubieCube) -> Self;
    /// Generate an arbitrary [`CubieCube`] which has the property defined by the coordinate. Any
    /// non specified property will not have a defined output.
    fn to_cubie_cube(&self) -> CubieCube;
}
