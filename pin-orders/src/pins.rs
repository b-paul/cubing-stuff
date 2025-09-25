use crate::matrix::ClockMatrix;

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
        use PinConfiguration as P;
        #[rustfmt::skip]
        const PINS: [PinConfiguration; 14] = [P::UR, P::DR, P::R, P::DL, P::FSLASH, P::D, P::NUL, P::UL, P::U, P::BSLASH, P::NDL, P::L, P::NDR, P::NUR];

        use itertools::Itertools;

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
}

#[test]
fn pinset_count() {
    assert_eq!(PinSet::all().count(), 268);
}
