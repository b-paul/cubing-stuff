// Stuff from https://cs.uwaterloo.ca/~astorjoh/diss2up.pdf
// We work over R = Z_12 since this is a PIR! Yay!

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
