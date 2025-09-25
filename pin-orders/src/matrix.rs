// these constants are stolen from my old 7 simul thingy

use crate::pins::PinOrder;

use nalgebra::SMatrix;

// moves are front face (index 2n) then back face (index 2n+1) indexed by the ordering of PINS
const MATRIX_ROWS: [[f64; 14]; 32] = [
    [0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 1., 0., 1., -1., -1., -1., -1., -1.],
    [0., 0., 0., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 1., 0., 0., -1., -1., -1., -1., -1.],
    [0., 1., 1., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 1., 0., 0., -1., 0., -1., -1., -1.],
    [0., 0., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 0., 0., 1., -1., -1., -1., -1., -1.],
    [0., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 0., 0., 1., -1., -1., -1., -1., -1.],
    [0., 0., 0., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 0., 0., 0., -1., -1., -1., -1., 0.],
    [0., 1., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., -1., -1., 0.],
    [1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 1., 0., 1., -1., -1., -1., -1., -1.],
    [1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 1., 0., 1., 0., -1., -1., -1., -1.],
    [1., 1., 0., 1., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 1., 0., 0., -1., -1., -1., -1., -1.],
    [1., 1., 1., 1., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., -1., -1.],
    [1., 1., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 0., 0., 1., -1., -1., -1., 0., -1.],
    [1., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 0., 0., 1., 0., -1., -1., 0., -1.],
    [1., 1., 0., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 0., 0., 0., -1., -1., -1., 0., 0.],
    [1., 1., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 1., 0., 1., -1., -1., -1., -1., -1.],
    [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
];

pub struct ClockMatrix(SMatrix<f64, 14, 14>);

impl ClockMatrix {
    pub fn from_pin_order(pin_order: PinOrder) -> ClockMatrix {
        // assume 7 simul everything for now i guess
        assert!(pin_order.0.len() == 7);
        let mut vec = Vec::with_capacity(196);

        for p in pin_order.0 {
            let mut temp1 = MATRIX_ROWS[2 * p as usize].to_vec();
            let mut temp2 = MATRIX_ROWS[2 * p as usize + 1].to_vec();

            vec.append(&mut temp1);
            vec.append(&mut temp2);
        }

        let matrix = SMatrix::<f64, 14, 14>::from_column_slice(&vec);

        ClockMatrix(matrix)
    }

    pub fn invertible(&self) -> bool {
        let det = self.0.determinant();

        // determinant of integer matrix is equal to determinant of it as a rationals matrix
        assert!((det - det.round()).abs() < 0.01);

        // determinant is a polynomial, so the projection hom preserves it into Z_12
        let det = (det.round() as i16).rem_euclid(12);

        // the determinant must be a unit in Z_12, i.e. coprime to 12
        det == 1 || det == 5 || det == 7 || det == 11
    }
}
