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
            matrix[2*i] = MATRIX_ROWS[2*p as usize];
            matrix[2*i + 1] = MATRIX_ROWS[2*p as usize + 1];
        }

        ClockMatrix(matrix)
    }

    fn to_nalgebra(&self) -> SMatrix::<f64, 14, 14> {
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
                matrix[i][j] = (inv[14*i + j].round() as i8 + 5).rem_euclid(12) - 5;
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
