use cube_lib::cube333::{
    moves::{Htm, Move333, MoveGenerator, Move333Type},
    CubieCube,
};
use rand::seq::IteratorRandom;
use std::collections::{HashSet, HashMap, VecDeque};

const U1: Move333 = Move333 {
    ty: Move333Type::U,
    count: 1,
};
const D1: Move333 = Move333 {
    ty: Move333Type::D,
    count: 1,
};
const R1: Move333 = Move333 {
    ty: Move333Type::R,
    count: 1,
};
const L1: Move333 = Move333 {
    ty: Move333Type::L,
    count: 1,
};
const F1: Move333 = Move333 {
    ty: Move333Type::F,
    count: 1,
};
const B1: Move333 = Move333 {
    ty: Move333Type::B,
    count: 2,
};
const U2: Move333 = Move333 {
    ty: Move333Type::U,
    count: 2,
};
const D2: Move333 = Move333 {
    ty: Move333Type::D,
    count: 2,
};
const R2: Move333 = Move333 {
    ty: Move333Type::R,
    count: 2,
};
const L2: Move333 = Move333 {
    ty: Move333Type::L,
    count: 2,
};
const F2: Move333 = Move333 {
    ty: Move333Type::F,
    count: 2,
};
const B2: Move333 = Move333 {
    ty: Move333Type::B,
    count: 2,
};
const U3: Move333 = Move333 {
    ty: Move333Type::U,
    count: 3,
};
const D3: Move333 = Move333 {
    ty: Move333Type::D,
    count: 3,
};
const R3: Move333 = Move333 {
    ty: Move333Type::R,
    count: 3,
};
const L3: Move333 = Move333 {
    ty: Move333Type::L,
    count: 3,
};
const F3: Move333 = Move333 {
    ty: Move333Type::F,
    count: 3,
};
const B3: Move333 = Move333 {
    ty: Move333Type::B,
    count: 3,
};

pub struct Fr;
pub struct Htr;

impl MoveGenerator for Fr {
    const SIZE: usize = 4;
    const MOVE_LIST: &'static [Move333] = &[R2, L2, F2, B2];
}

impl MoveGenerator for Htr {
    const SIZE: usize = 6;
    const MOVE_LIST: &'static [Move333] = &[U2, D2, R2, L2, F2, B2];
}

#[derive(Debug)]
struct GeneratorSet {
    set: HashSet<CubieCube>,
}

impl GeneratorSet {
    fn from_generator_trait<G: MoveGenerator>(cube: CubieCube) -> Self {
        // BFS on solved cube
        let mut set = HashSet::new();

        let mut stack = vec![cube];

        while let Some(cube) = stack.pop() {
            set.insert(cube.clone());

            for &mv in G::MOVE_LIST {
                let new_cube = cube.make_move(mv);
                if !set.contains(&new_cube) {
                    stack.push(new_cube);
                }
            }
        }

        GeneratorSet { set }
    }

    fn from_generator_trait_print_solns<G: MoveGenerator>(cube: CubieCube) -> Self {
        // BFS on solved cube
        let mut set = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back((cube.clone(), vec![]));
        set.insert(cube);

        while let Some((cube, soln)) = queue.pop_front() {
            println!("{}", soln.iter().cloned().map(|mv| format!("{mv:?}")).collect::<Vec<_>>().join(" "));

            for &mv in G::MOVE_LIST {
                let new_cube = cube.make_move(mv);
                if !set.contains(&new_cube) {
                    set.insert(new_cube.clone());
                    let mut soln2 = soln.clone();
                    soln2.push(mv);
                    queue.push_back((new_cube, soln2));
                }
            }
        }

        GeneratorSet { set }
    }

    fn solved_set() -> Self {
        let mut set = HashSet::new();
        set.insert(CubieCube::SOLVED);
        GeneratorSet { set }
    }

    fn slice_set() -> Self {
        let mut set = HashSet::new();

        set.insert(CubieCube::SOLVED);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(R2);
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        cube = cube.make_move(B2);
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        cube = cube.make_move(B2);
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        cube = cube.make_move(R2);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(R2);
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        cube = cube.make_move(F2);
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        cube = cube.make_move(F2);
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        cube = cube.make_move(R2);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(L2);
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        cube = cube.make_move(B2);
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        cube = cube.make_move(B2);
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        cube = cube.make_move(L2);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(L2);
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        cube = cube.make_move(F2);
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(U3);
        cube = cube.make_move(D1);
        cube = cube.make_move(F2);
        cube = cube.make_move(U1);
        cube = cube.make_move(D3);
        cube = cube.make_move(L2);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(L2);
        cube = cube.make_move(U2);
        cube = cube.make_move(D2);
        cube = cube.make_move(R2);
        cube = cube.make_move(U2);
        cube = cube.make_move(D2);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(F2);
        cube = cube.make_move(U2);
        cube = cube.make_move(D2);
        cube = cube.make_move(B2);
        cube = cube.make_move(U2);
        cube = cube.make_move(D2);
        set.insert(cube);

        let mut cube = CubieCube::SOLVED;
        cube = cube.make_move(R2);
        cube = cube.make_move(F2);
        cube = cube.make_move(U2);
        cube = cube.make_move(D2);
        cube = cube.make_move(B2);
        cube = cube.make_move(U2);
        cube = cube.make_move(D2);
        cube = cube.make_move(R2);
        set.insert(cube);

        GeneratorSet { set }
    }

    fn product<G: MoveGenerator>(&self) -> Self {
        let mut set = HashSet::new();
        for cube in self.set.clone().into_iter() {
            let tmpset = Self::from_generator_trait::<G>(cube);
            for cube in tmpset.set.into_iter() {
                set.insert(cube);
            }
        }
        GeneratorSet { set }
    }
}

fn search<G: MoveGenerator>(
    cube: CubieCube,
    set: &GeneratorSet,
    depth: usize,
) -> Option<Vec<Move333>> {
    if depth == 0 {
        if set.set.contains(&cube) {
            return Some(vec![]);
        }
        return None;
    }
    for mv in G::MOVE_LIST {
        let new_cube = cube.make_move(*mv);
        if let Some(mut seq) = search::<G>(new_cube, set, depth - 1) {
            seq.push(*mv);
            return Some(seq);
        }
    }
    None
}

fn solve<G: MoveGenerator>(cube: CubieCube, set: &GeneratorSet) -> Vec<Move333> {
    for depth in 0..32 {
        if let Some(mut sol) = search::<G>(cube.clone(), set, depth) {
            sol.reverse();
            return sol;
        }
    }
    panic!("Cube took way too long to solve");
}

fn main() {
    let solved_set = GeneratorSet::solved_set();
    let slice_set = GeneratorSet::slice_set();
    let fr_set = GeneratorSet::from_generator_trait::<Fr>(CubieCube::SOLVED);
    let fr_slice_set = slice_set.product::<Fr>();
    //println!("Floppy reduction set generated with {} elements", fr_set.set.len());
    let htr_set = GeneratorSet::from_generator_trait_print_solns::<Htr>(CubieCube::SOLVED);
    //println!("HTR set generated with {} elements", htr_set.set.len());

    /*
    let mut cube = CubieCube::SOLVED;

    cube = cube.make_move(B2);
    cube = cube.make_move(L2);
    cube = cube.make_move(B2);
    cube = cube.make_move(R2);
    cube = cube.make_move(L2);
    cube = cube.make_move(B2);
    cube = cube.make_move(L2);
    cube = cube.make_move(F2);

    let seq = solve::<Htm>(cube, &solved_set);
    println!("{:?}", seq);
    */

    /*
    for cube in htr_set.set.iter() {
        let seq = solve::<Htr>(cube.clone(), &fr_slice_set);
        println!("{:?}", seq);
    }
    */
}
