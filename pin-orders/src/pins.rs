use crate::matrix::{ClockMatrix, CompletedMatrix};
use crate::z12::Z12;

use std::collections::BTreeSet;

/// All of the ways you can have the pins configured on a clock, assuming a fixed rotation and side
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum PinConfiguration {
    UR = 0,
    DR = 1,
    R = 2,
    DL = 3,
    // /
    FSLASH = 4,
    D = 5,
    NUL = 6,
    UL = 7,
    U = 8,
    // \
    BSLASH = 9,
    NDL = 10,
    L = 11,
    NDR = 12,
    NUR = 13,
    //ALL = 14,
    //NONE = 15,
}

impl PinConfiguration {
    pub fn to_bitmask(self) -> u8 {
        // UR 0th DR 1st DL 2nd UL 3rd
        (self as u8 + 1) & 15
    }
}

impl std::fmt::Display for PinConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinConfiguration::UR => write!(f, "UR"),
            PinConfiguration::DR => write!(f, "DR"),
            PinConfiguration::R => write!(f, "R"),
            PinConfiguration::DL => write!(f, "DL"),
            PinConfiguration::FSLASH => write!(f, "/"),
            PinConfiguration::D => write!(f, "D"),
            PinConfiguration::NUL => write!(f, "ul"),
            PinConfiguration::UL => write!(f, "UL"),
            PinConfiguration::U => write!(f, "U"),
            PinConfiguration::BSLASH => write!(f, "\\"),
            PinConfiguration::NDL => write!(f, "dl"),
            PinConfiguration::L => write!(f, "L"),
            PinConfiguration::NDR => write!(f, "dr"),
            PinConfiguration::NUR => write!(f, "ur"),
            //PinConfiguration::ALL => write!(f, "ALL"),
            //PinConfiguration::NONE => write!(f, "NONE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Piece {
    UL = 0,
    U = 1,
    UR = 2,
    L = 3,
    C = 4,
    R = 5,
    DL = 6,
    D = 7,
    DR = 8,
    BU = 9,
    BL = 10,
    BC = 11,
    BR = 12,
    BD = 13,
}

#[rustfmt::skip]
const PIECES: [Piece; 14] = [Piece::UL, Piece::U, Piece::UR, Piece::L, Piece::C, Piece::R, Piece::DL, Piece::D, Piece::DR, Piece::BU, Piece::BL, Piece::BC, Piece::BR, Piece::BD];

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::UL => write!(f, "UL"),
            Piece::U => write!(f, "U"),
            Piece::UR => write!(f, "UR"),
            Piece::L => write!(f, "L"),
            Piece::C => write!(f, "C"),
            Piece::R => write!(f, "R"),
            Piece::DL => write!(f, "DL"),
            Piece::D => write!(f, "D"),
            Piece::DR => write!(f, "DR"),
            Piece::BU => write!(f, "u"),
            Piece::BL => write!(f, "l"),
            Piece::BC => write!(f, "c"),
            Piece::BR => write!(f, "r"),
            Piece::BD => write!(f, "d"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PinSet(pub BTreeSet<PinConfiguration>);

impl PinSet {
    pub fn into_pin_order(self) -> PinOrder {
        let mut v = self.0.into_iter().collect::<Vec<_>>();
        v.sort();
        PinOrder(v)
    }

    /// An iterator over all valid 7 simul pin sets
    pub fn all() -> impl Iterator<Item = Self> {
        use itertools::Itertools;

        use PinConfiguration as P;
        #[rustfmt::skip]
        const PINS: [PinConfiguration; 14] = [P::UR, P::DR, P::R, P::DL, P::FSLASH, P::D, P::NUL, P::UL, P::U, P::BSLASH, P::NDL, P::L, P::NDR, P::NUR];

        PINS.into_iter().combinations(7).filter_map(|c| {
            PinOrder(c.clone())
                .as_matrix()
                .invertible()
                .then_some(PinSet(c.into_iter().collect()))
        })
    }
}

impl std::fmt::Display for PinOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PinOrder(pub Vec<PinConfiguration>);

impl PinOrder {
    pub fn as_matrix(&self) -> ClockMatrix {
        ClockMatrix::from_pin_order(self)
    }

    pub fn count_transitions(&self) -> u32 {
        let cur = self.0[0];
        self.0
            .iter()
            .skip(1)
            .fold((0, cur.to_bitmask()), |(count, cur), next| {
                (
                    count + (cur ^ next.to_bitmask()).count_ones(),
                    next.to_bitmask(),
                )
            })
            .0
    }

    pub fn make_tutorial(
        &self,
        f: &mut impl std::io::Write,
        memo: [MoveSolution; 14],
    ) -> std::io::Result<()> {
        writeln!(f, "\nPin order: {self}")?;

        fn formula(f: &mut impl std::io::Write, memo: MoveSolution) -> std::io::Result<()> {
            match memo {
                MoveSolution::Memo(row) => {
                    for (i, n) in row.into_iter().enumerate() {
                        if n == 0 {
                            continue;
                        }
                        if n < 0 {
                            write!(f, "+")?;
                        } else if n > 0 {
                            write!(f, "-")?;
                        }
                        if n.abs() == 1 {
                            write!(f, "{}", PIECES[i])?;
                        } else if n.abs() > 1 {
                            write!(f, "{}{}", n.abs(), PIECES[i])?;
                        }
                    }
                }
                MoveSolution::Intuitive { from, to } => write!(f, "{from} to {to}")?,
                MoveSolution::Obvious => write!(f, "finish")?,
            }
            Ok(())
        }

        for (i, pin) in self.0.iter().enumerate() {
            write!(f, "{pin}: (")?;
            formula(f, memo[2 * i])?;
            write!(f, ", ")?;
            formula(f, memo[2 * i + 1])?;
            writeln!(f, ")")?;
        }
        writeln!(f, "\n")?;

        Ok(())
    }

    pub fn gen_solution(&self, scramble: [i8; 14]) -> [i8; 14] {
        let inv = self.as_matrix().try_inverse().unwrap().0;

        let mut r = [0; 14];

        for i in 0..14 {
            for (j, &s) in scramble.iter().enumerate() {
                r[i] -= inv[i][j] * s;
            }
            r[i] = (r[i] + 5).rem_euclid(12) - 5;
        }

        r
    }
}

#[test]
fn pinset_count() {
    assert_eq!(PinSet::all().count(), 268);
}

// Generating reduced memo:
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MoveSolution {
    /// A move that has to be calculated from a formula specified by this row vector.
    Memo([i8; 14]),
    /// An intuitive move, done by making `from` line up with `to`.
    Intuitive {
        from: Piece,
        to: Piece,
    },
    // The last move lol
    Obvious,
}

impl PinOrder {
    pub fn gen_memo(&self) -> [MoveSolution; 14] {
        let mut arr = [MoveSolution::Obvious; 14];

        let mut completed_matrix = CompletedMatrix(Vec::new());

        let mat = self
            .as_matrix()
            .try_inverse()
            .expect("Tried to generate memo for an invalid pin order");

        println!("{self}");
        for i in 0..6 {
            for (j, memo) in arr[2 * i..=2 * i + 1].iter_mut().enumerate() {
                *memo = if let Some((from, to)) =
                    completed_matrix.find_intuitive(mat.0[2*i + j].map(Z12::new))
                {
                    MoveSolution::Intuitive {
                        from: PIECES[from],
                        to: PIECES[to],
                    }
                } else {
                    MoveSolution::Memo(mat.0[2*i + j])
                };
            }

            completed_matrix.0.push(mat.0[2 * i].map(Z12::new));
            completed_matrix.0.push(mat.0[2 * i + 1].map(Z12::new));
        }

        arr
    }
}
