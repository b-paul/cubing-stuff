//! A library which implements various twisty puzzles and helpful functions
//! related to them. This is mostly for personal use.

#![deny(missing_docs)]
#![feature(array_try_map)]

/// Module for implementations of functionality generic over many different puzzles.
pub mod generic;
/// Module for the 3x3x3 Rubik's cube (the one everyone knows).
pub mod cube333;
