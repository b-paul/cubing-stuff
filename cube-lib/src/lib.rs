//! A library which implements various twisty puzzles and helpful functions
//! related to them. This is mostly for personal use.

#![deny(missing_docs)]
#![feature(slice_group_by)]

use thiserror::Error;

// this shouldnt be in lib.rs but im lazy, will move it later i guess lol
// it would be nice if there was a way to include the name of the enum in the error message!
/// Error type for converting integers to (C like) enums using TryFrom
#[derive(Debug, Error)]
pub enum TryFromIntToEnumError  {
    /// attempted to convert integer into enum value, but integer was out of bounds
    #[error("attempted to convert integer into enum value, but integer was out of bounds")]
    OutOfBounds
}

pub mod cube333;
pub mod moves;
