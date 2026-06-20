// 1. Represent bandaged cubes
//    - DONE Sets of masks of glued pieces
//    - DONE Conversion from mask to set of piece
//    - DONE Apply symmetries to bandages
//        - TODO memoised
//    - DONE Subsets of bandages based on what is glued
//    - DONE ^ This mod symmetry
//    - Decide whether to store masks per piece ?!?!
//    - Make moves on bandaged cubes
//       - Join masks of all pieces affected by a move
//       - if it is not a subset of the move mask, it is invalid! else it is valid
//    - Explore graph of a bandaging
//    - Self solving property???? or maybe do this later
//    - Iterate over all cuts to a piece along move axes
// 2. the algorithm:
//    - Priority queue ordered by cardinality of bandage graph
//    - store a map from cardinalities to list of bandagings.
//        - extend by deduping with subset comparisons mod symmetry
//    - store union find like thing maybe?!?!?!?!??!
//    - pop from priority queue
//    - check if an equivalent version of this bandaging has already been run
//        - maybe can store a list of iterated bandages of this current cardinality ?
//    - iterate over all single cuts from this bandage
//    - add to map, if new add to priority queue ? (could maybe delay adding to priority queue)
// Later:
//    - slices

use cube_lib::cube333::{
    CubieCube,
    corner::Corner,
    edge::Edge,
    moves::{Move333, Move333Type, Htm, MoveGenerator},
    sym::CubeSymmetry,
};

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Piece {
    Corner(Corner),
    Edge(Edge),
    // Add centres later
}

impl Piece {
    fn idx(self) -> usize {
        match self {
            Piece::Corner(corner) => corner as usize,
            Piece::Edge(edge) => edge as usize + 8,
        }
    }

    fn of_idx(i: u32) -> Piece {
        if i < 8 {
            Piece::Corner(Corner::try_from(i as u8).unwrap())
        } else {
            Piece::Edge(Edge::try_from(i as u8 - 8).unwrap())
        }
    }

    fn apply_symmetry(self, s: CubeSymmetry) -> Piece {
        match self {
            Piece::Corner(corner) => Piece::Corner(s.transform_cp(corner)),
            Piece::Edge(edge) => Piece::Edge(s.transform_ep(edge)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BandageMask(u32);

impl BandageMask {
    fn to_set(self) -> HashSet<Piece> {
        let mut s = HashSet::new();
        let mut m = self.0;
        while let Some(i) = m.lowest_one() {
            s.insert(Piece::of_idx(i));
        }
        s
    }

    fn of_set(s: HashSet<Piece>) -> Self {
        BandageMask(s.into_iter().fold(0, |m, p| m | (1 << p.idx())))
    }

    /// Mask of pieces affected by a move
    fn affected_pieces(m: Move333Type) -> Self {
        match m {
            Move333Type::R => BandageMask(0x98869),
            Move333Type::L => BandageMask(0x62296),
            Move333Type::U => BandageMask(0x00f0f),
            Move333Type::D => BandageMask(0x0f0f0),
            Move333Type::F => BandageMask(0x31133),
            Move333Type::B => BandageMask(0xc44cc),
        }
    }

    /// Determines if the left pieces is a subset of the right pieces
    fn subset(self, other: Self) -> bool {
        self.0 | other.0 == other.0
    }

    fn disjoint(self, other: Self) -> bool {
        self.0 & other.0 == 0
    }

    fn apply_symmetry(self, s: CubeSymmetry) -> Self {
        Self::of_set(
            self.to_set()
                .into_iter()
                .map(|p| p.apply_symmetry(s))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bandaging {
    masks: Vec<BandageMask>,
}

impl Bandaging {
    fn disjoint(&self) -> bool {
        self.masks
            .iter()
            .fold((true, 0), |(acc, m), BandageMask(m2)| {
                (acc && (m & m2 == 0), m | m2)
            })
            .0
    }

    // TODO memoise
    fn apply_symmetry(self, s: CubeSymmetry) -> Self {
        let masks = self
            .masks
            .into_iter()
            .map(|m| m.apply_symmetry(s))
            .collect();
        Self { masks }
    }

    fn subset(&self, other: &Bandaging) -> bool {
        self.masks
            .iter()
            .all(|&m| other.masks.iter().any(|&m2| m.subset(m2)))
    }

    // Determines whether a symmetry of this bandaging is a subset of the other bandaging
    fn sym_subset(&self, other: &Bandaging) -> bool {
        CubeSymmetry::all().any(|s| self.clone().apply_symmetry(s).subset(other))
    }

    /// Count how many states exist in the state graph of this bandaging.
    fn states(&self) -> usize {
        let init = BandageCube {
            bandaging: self.clone(),
            cube: CubieCube::SOLVED,
        };
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        visited.insert(CubieCube::SOLVED);
        stack.push(init);
        let mut count = 0;

        while let Some(cur) = stack.pop() {
            count += 1;
            for &mv in Htm::MOVE_LIST {
                if let Some(next) = cur.clone().try_make_move(mv) {
                    todo!()
                }
            }
        }

        count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BandageCube {
    bandaging: Bandaging,
    cube: CubieCube,
}

impl BandageCube {
    // Drops a potential copy but I can't be bothered to deal with Results...
    fn try_make_move(self, mv: Move333) -> Option<Self> {
        let affected_locations = BandageMask::affected_pieces(mv.ty);
        let affected_pieces = BandageMask::of_set(
            affected_locations
                .to_set()
                .into_iter()
                .map(|p| match p {
                    Piece::Corner(corner) => Piece::Corner(self.cube.cp[corner as usize]),
                    Piece::Edge(edge) => Piece::Edge(self.cube.ep[edge as usize]),
                })
                .collect(),
        );
        if self
            .bandaging
            .masks
            .iter()
            .all(|&m| m.disjoint(affected_pieces) || m.subset(affected_pieces))
        {
            None
        } else {
            let Self { bandaging, cube } = self;
            Some(Self {
                bandaging,
                cube: cube.make_move(mv),
            })
        }
    }
}

fn main() {
    println!("Hello, world!");
}
