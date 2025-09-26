// these constants are stolen from my old 7 simul thingy

use std::fmt::Display;

use crate::pins::PinOrder;

use nalgebra::SMatrix;

// moves are front face (index 2n) then back face (index 2n+1) indexed by the ordering of PINS
const MATRIX_ROWS: [[i8; 14]; 32] = [
    [0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 1, 0, 1, -1, -1, -1, -1, -1],
    [0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 1, 0, 0, -1, -1, -1, -1, -1],
    [0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, -1, 0, -1],
    [0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0, 1, -1, -1, -1, -1, -1],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, -1, -1, -1, -1, -1],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1],
    [1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 1, -1, -1, -1, -1, -1],
    [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 1, -1, -1, -1, -1, 0],
    [1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 0, -1, -1, -1, -1, -1],
    [1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, -1, 0, 0],
    [1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 1, -1, 0, -1, -1, -1],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, -1, -1, 0],
    [1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 1, 0, 1, -1, -1, -1, -1, -1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

pub struct ClockMatrix(pub [[i8; 14]; 14]);

impl ClockMatrix {
    pub fn from_pin_order(pin_order: &PinOrder) -> ClockMatrix {
        // assume 7 simul everything for now i guess
        assert!(pin_order.0.len() == 7);
        let mut matrix = [[0; 14]; 14];

        for (i, &p) in pin_order.0.iter().enumerate() {
            matrix[2 * i] = MATRIX_ROWS[2 * p as usize];
            matrix[2 * i + 1] = MATRIX_ROWS[2 * p as usize + 1];
        }

        ClockMatrix(matrix)
    }

    fn to_nalgebra(&self) -> SMatrix<f64, 14, 14> {
        let mut vec = Vec::with_capacity(196);

        for &s in &self.0 {
            vec.extend(s.map(|i| i as f64));
        }

        SMatrix::<f64, 14, 14>::from_row_slice(&vec)
    }

    pub fn invertible(&self) -> bool {
        let det = self.to_nalgebra().determinant();

        // determinant of integer matrix is equal to determinant of it as a rationals matrix
        assert!((det - det.round()).abs() < 0.01);

        // determinant is a polynomial, so the projection hom preserves it into Z_12
        let det = (det.round() as i16).rem_euclid(12);

        // the determinant must be a unit in Z_12, i.e. coprime to 12
        det == 1 || det == 5 || det == 7 || det == 11
    }

    pub fn try_inverse(&self) -> Option<ClockMatrix> {
        let inv = self.to_nalgebra().try_inverse()?;

        let mut matrix = [[0; 14]; 14];

        for i in 0..14 {
            for j in 0..14 {
                // we do this add and subtract by 5 to involve negative numbers on th boundaries
                // we add 5 so that 6 is represented as positive 6.
                matrix[i][j] = (inv[14 * i + j].round() as i8 + 5).rem_euclid(12) - 5;
            }
        }

        Some(ClockMatrix(matrix))
    }
}

impl Display for ClockMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_nalgebra())
    }
}

// Stuff from https://cs.uwaterloo.ca/~astorjoh/diss2up.pdf
// We work over R = Z_12 since this is a PIR! Yay!
// The units of Z_12 are {1, 5, 7, 11}.
// We choose the nonassociates to be {0, 1, 2, 3, 4, 5, 6}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Z12(i8);

impl Z12 {
    fn is_unit(self) -> bool {
        [1, 5, 7, 11].contains(&self.0)
    }
}

impl std::fmt::Display for Z12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 + 5).rem_euclid(12) - 5)
    }
}

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Z12 {
    type Output = Z12;

    fn add(self, rhs: Self) -> Self::Output {
        Z12((self.0 + rhs.0).rem_euclid(12))
    }
}

impl AddAssign for Z12 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Z12 {
    type Output = Z12;

    fn sub(self, rhs: Self) -> Self::Output {
        Z12((self.0 - rhs.0).rem_euclid(12))
    }
}

impl SubAssign for Z12 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Z12 {
    type Output = Z12;

    fn mul(self, rhs: Self) -> Self::Output {
        Z12((self.0 * rhs.0).rem_euclid(12))
    }
}

impl MulAssign for Z12 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Returns g, s, t, u, v such that sv - tu is a unit, sa + tb = g and ua + vt = 0.
fn gcdex(a: Z12, b: Z12) -> (Z12, Z12, Z12, Z12, Z12) {
    // code from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode lol

    let (mut old_r, mut r) = (a.0, b.0);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (
        Z12(old_r.rem_euclid(12)),
        Z12(old_s.rem_euclid(12)),
        Z12(old_t.rem_euclid(12)),
        Z12(s.rem_euclid(12)),
        Z12(t.rem_euclid(12)),
    )
}

#[test]
fn z12_tests() {
    for i in 0..12 {
        for j in 0..12 {
            let (a, b) = (Z12(i), Z12(j));
            let (g, s, t, u, v) = gcdex(a, b);
            assert!((s * v - t * u).is_unit());
            assert_eq!(s * a + t * b, g);
            assert_eq!(u * a + v * b, Z12(0));
        }
    }
}
