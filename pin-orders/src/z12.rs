// Stuff from https://cs.uwaterloo.ca/~astorjoh/diss2up.pdf
// We work over R = Z_12 since this is a PIR! Yay!

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Z12(i8);

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Z12 {
    type Output = Z12;

    fn add(self, rhs: Self) -> Self::Output {
        Z12::new(self.0 + rhs.0)
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
        Z12::new(self.0 - rhs.0)
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
        Z12::new(self.0 * rhs.0)
    }
}

impl MulAssign for Z12 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Z12 {
    pub fn new(x: i8) -> Z12 {
        Z12(x.rem_euclid(12))
    }

    pub fn is_unit(self) -> bool {
        [1, 5, 7, 11].contains(&self.0)
    }

    /// Returns g, s, t, u, v such that sv - tu is a unit, sa + tb = g and ua + vt = 0.
    pub fn gcdex(self, b: Z12) -> (Z12, Z12, Z12, Z12, Z12) {
        // code from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode lol

        let (mut old_r, mut r) = (self.0, b.0);
        let (mut old_s, mut s) = (1, 0);
        let (mut old_t, mut t) = (0, 1);

        while r != 0 {
            let q = old_r / r;
            (old_r, r) = (r, old_r - q * r);
            (old_s, s) = (s, old_s - q * s);
            (old_t, t) = (t, old_t - q * t);
        }

        (
            Z12::new(old_r),
            Z12::new(old_s),
            Z12::new(old_t),
            Z12::new(s),
            Z12::new(t),
        )
    }
}

impl std::fmt::Display for Z12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 + 5).rem_euclid(12) - 5)
    }
}

#[test]
fn z12_tests() {
    for i in 0..12 {
        for j in 0..12 {
            let (a, b) = (Z12::new(i), Z12::new(j));
            let (g, s, t, u, v) = a.gcdex(b);
            assert!((s * v - t * u).is_unit());
            assert_eq!(s * a + t * b, g);
            assert_eq!(u * a + v * b, Z12(0));
        }
    }
}
