use super::CubieCube;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct COCoord(u16);
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct CPCoord(u16);
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct EOCoord(u16);
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct EPCoord(u32);

impl From<COCoord> for usize {
    fn from(coord: COCoord) -> usize {
        coord.0 as usize
    }
}
impl From<CPCoord> for usize {
    fn from(coord: CPCoord) -> usize {
        coord.0 as usize
    }
}
impl From<EOCoord> for usize {
    fn from(coord: EOCoord) -> usize {
        coord.0 as usize
    }
}
impl From<EPCoord> for usize {
    fn from(coord: EPCoord) -> usize {
        coord.0 as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CoordCube {
    pub co: COCoord,
    pub cp: CPCoord,
    pub eo: EOCoord,
    pub ep: EPCoord,
}

impl CoordCube {
    pub fn to_cubie(&self) -> CubieCube {
        todo!()
    }

    pub fn solved() -> Self {
        CoordCube {
            co: COCoord(0),
            cp: CPCoord(0),
            eo: EOCoord(0),
            ep: EPCoord(0),
        }
    }
}

impl CubieCube {
    pub fn to_coord(&self) -> CoordCube {
        let co = COCoord(Self::to_o_coord::<8, 3>(&self.co));
        let cp = CPCoord(Self::to_p_coord::<8>(&self.cp) as u16);
        let eo = EOCoord(Self::to_o_coord::<12, 2>(&self.eo));
        let ep = EPCoord(Self::to_p_coord::<12>(&self.ep));

        CoordCube { co, cp, eo, ep }
    }

    fn to_o_coord<const COUNT: usize, const STATES: u16>(arr: &[u8; COUNT]) -> u16 {
        arr.iter()
            .skip(1)
            .fold(0, |acc, &i| (acc * STATES) + i as u16)
    }

    fn to_p_coord<const COUNT: usize>(_arr: &[u8; COUNT]) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn invert_identity() {
        let identity = CoordCube::solved();
        assert_eq!(identity, identity.to_cubie().to_coord())
    }
    */

    use crate::cube333::moves::{Htm, Move, MoveGenerator};
    use proptest::strategy::{Strategy, ValueTree};
    use proptest::test_runner::TestRunner;
    #[test]
    fn moves_work() {
        // proptest
        let mut runner = TestRunner::default();
        for _ in 0..256 {
            let mut mvs = vec![];
            for _ in 0..(0..40usize).new_tree(&mut runner).unwrap().current() {
                let val = (0..18usize).new_tree(&mut runner).unwrap();
                let mv = Htm::MOVE_LIST[val.current()];
                mvs.push(mv);
            }
            println!("{:?}", mvs);
            // Now we want to apply the sequence of moves to a cubiecube and a coordinatecube made
            // from that cubie cube, then convert the resulting coordinatecube into a cubiecube and
            // check if the two cubiecubes are the same
        }
    }
}

/*
impl CoordCube {
    #[inline]
    fn to_cubie_o<const SIZE: usize>(mut cord: u16, and: u8, modulo: u8, shift: u8) -> [u8; SIZE] {
        let mut o = [0; SIZE];
        let mut sum = 0;
        for o in o.iter_mut().take(SIZE - 2) {
            let digit: u16 = cord & (and as u16);
            for i in 0..(and - 1) {
                if digit == u16::from(i) {
                    *o = i;
                    sum += i;
                    break;
                }
            }
            cord >>= shift;
        }
        o[SIZE - 1] = sum % modulo;
        o
    }

    // Add error handling maybe? idk
    // Error handling should be done on cubie creation
    pub fn to_cubie(&self) -> CubieCube {
        let co: [u8; 8] = CoordCube::to_cubie_o(self.co, 3, 3, 2);
        let eo: [u8; 12] = CoordCube::to_cubie_o(self.eo, 1, 2, 1);

        // CP/EP
        // First digit is a number from 0-7
        // Next digit is a number from 0-6 (exclude the previous number)
        // etc
        // looking to de duplicate ize this
        let mut cp: [u8; 8] = [0; 8];
        let mut cpcord = self.cp;
        let mut v: Vec<usize> = (0..8).collect();
        for index in (0..8).rev() {
            cp[index] = v.remove(cpcord as usize % (index + 1)) as u8;
            cpcord /= (index + 1) as u16;
        }

        let mut ep: [u8; 12] = [0; 12];
        let mut epcord = self.ep;
        let mut v: Vec<usize> = (0..12).collect();
        for index in (0..12).rev() {
            ep[index] = v.remove(epcord as usize % (index + 1)) as u8;
            epcord /= (index + 1) as u32;
        }

        CubieCube { co, cp, eo, ep }
    }
}

impl CubieCube {
    pub fn to_coord_o<const SIZE: usize>(cubie: [u8; SIZE], shift: usize) -> u16 {
        let mut o: u16 = 0;
        for i in cubie {
            o += i as u16;

            o <<= shift;
        }

        o
    }

    pub fn to_coord(&self) -> CoordCube {
        let co = CubieCube::to_coord_o(self.co, 2);
        let eo = CubieCube::to_coord_o(self.eo, 1);

        // yikes bad code
        let mut cp: u16 = 0;
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7];

        for index in (0..8).rev() {
            let pos = v.iter().position(|&r| r == self.cp[index] as u16).unwrap() as u16;
            cp += pos;
            v.remove(pos as usize);
            cp *= (index + 1) as u16;
        }

        let mut ep: u32 = 0;
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        for index in (0..12).rev() {
            let pos = v.iter().position(|&r| r == self.ep[index] as u16).unwrap() as u32;
            ep += pos;
            v.remove(pos as usize);
            ep *= (index + 1) as u32;
        }

        CoordCube { co, cp, eo, ep }
    }
}
*/
