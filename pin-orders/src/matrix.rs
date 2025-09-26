// these constants are stolen from my old 7 simul thingy

use std::fmt::Display;

use crate::pins::PinOrder;

use nalgebra::{DMatrix, SMatrix};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

use crate::z12::Z12;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompletedMatrix(pub Vec<[Z12; 14]>);

impl CompletedMatrix {
    pub fn to_nalgebra(&self) -> DMatrix<f64> {
        let mut vec = Vec::with_capacity(self.0.len() * 14);

        for &s in &self.0 {
            vec.extend(s.map(|i| ((i.0 + 5).rem_euclid(12) - 5) as f64));
        }

        DMatrix::from_row_slice(self.0.len(), 14, &vec)
    }

    pub fn to_echelon(mut self) -> CompletedMatrix {
        if self.0.len() == 0 {
            return self;
        }

        let mut r = 0;

        for k in 0..14 {
            for i in r + 1..self.0.len() {
                let (_, s, t, u, v) = self.0[r][k].gcdex(self.0[i][k]);
                for j in 0..14 {
                    let (a, b) = (self.0[r][j], self.0[i][j]);
                    self.0[r][j] = s * a + t * b;
                    self.0[i][j] = u * a + v * b;
                }
            }
            if self.0[r][k] != Z12::new(0) {
                r += 1;
            }
            if r >= self.0.len() {
                break;
            }
        }

        self
    }

    // This assumes self is in echelon form so you better make sure it is !!!
    pub fn check_intuitive(&self, mut row: [Z12; 14], from: usize, to: usize) -> bool {
        // The row is meant to be negated but we can just negate the from and to terms we add
        // instead except idk
        row[from] -= Z12(1);
        row[to] += Z12(1);

        let mut x = Vec::new();
        let mut pivot = 0;

        for r in 0..self.0.len() {
            // i don't think this should panic...
            while self.0[r][pivot] == Z12::new(0) {
                pivot += 1;
            }
            let mut val = row[pivot];
            for (j, &v) in x.iter().enumerate() {
                val -= v * self.0[j][pivot];
            }
            // solve for x[r] in row[r] = sum x[i] self.0[i][pivot]
            // x[r] self.0[r][pivot] = row[r] - sum^r-1 x[i]
            // so divide by self.0[r][pivot]
            let Some(x2) = val.divide(self.0[r][pivot]) else {
                return false;
            };
            x.push(x2);
        }

        for (i, r) in row.into_iter().enumerate() {
            if (0..self.0.len()).map(|j| x[j] * self.0[j][i]).sum::<Z12>() != r {
                return false;
            }
        }

        true
    }

    pub fn find_intuitive(&self, row: [Z12; 14]) -> Option<(usize, usize)> {
        let echelon = self.clone().to_echelon();
        for from in 0..9 {
            for to in 0..9 {
                if to == from {
                    continue;
                }
                if echelon.check_intuitive(row, from, to) {
                    return Some((from, to));
                }
            }
        }
        None
    }
}

impl Display for CompletedMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_nalgebra())
    }
}
