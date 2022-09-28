use super::coordinate::Coordinate;
use super::moves::Htm;
use super::CubieCube;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct COCoord(u16);
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct CPCoord(u16);
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct EOCoord(u16);
#[derive(Debug, Default, PartialEq, Copy, Clone)]
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

impl Coordinate for COCoord {
    type Generator = Htm;
    // 3^7
    const SIZE: usize = 2187;

    fn from_cubie_cube(cube: &CubieCube) -> Self {
        let mut co: u16 = 0;
        for i in cube.co {
            co += i as u16;

            co <<= 2;
        }

        COCoord(co)
    }

    fn to_cubie_cube(&self) -> CubieCube {
        let mut coord = self.0;

        let mut co = [0; 8];
        let mut sum = 0;
        for co in co.iter_mut().take(7) {
            let digit = (coord & 3) as u8;
            *co = digit;
            sum += digit;
            coord >>= 2;
        }
        co[7] = sum % 3;
        CubieCube {
            co,
            cp: [0; 8],
            eo: [0; 12],
            ep: [0; 12],
        }
    }
}

impl Coordinate for CPCoord {
    type Generator = Htm;
    // 8!/2
    const SIZE: usize = 20160;

    fn from_cubie_cube(cube: &CubieCube) -> Self {
        // Please please reimplement this!!! and EP as well
        let mut cp: u16 = 0;
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7];

        for index in (0..8).rev() {
            let pos = v.iter().position(|&r| r == cube.cp[index] as u16).unwrap() as u16;
            cp += pos;
            v.remove(pos as usize);
            cp *= (index + 1) as u16;
        }

        CPCoord(cp)
    }

    fn to_cubie_cube(&self) -> CubieCube {
        let mut cp: [u8; 8] = [0; 8];
        let mut cpcord = self.0;
        let mut v: Vec<usize> = (0..8).collect();
        for index in (0..8).rev() {
            cp[index] = v.remove(cpcord as usize % (index + 1)) as u8;
            cpcord /= (index + 1) as u16;
        }
        CubieCube {
            co: [0; 8],
            cp,
            eo: [0; 12],
            ep: [0; 12],
        }
    }
}

impl Coordinate for EOCoord {
    type Generator = Htm;
    // 2^11
    const SIZE: usize = 2048;

    fn from_cubie_cube(cube: &CubieCube) -> Self {
        let mut eo: u16 = 0;
        for i in cube.eo {
            eo += i as u16;

            eo <<= 1;
        }

        EOCoord(eo)
    }

    fn to_cubie_cube(&self) -> CubieCube {
        let mut coord = self.0;

        let mut eo = [0; 12];
        let mut sum = 0;
        for eo in eo.iter_mut().take(11) {
            let digit = (coord & 1) as u8;
            *eo = digit;
            sum += digit;
            coord >>= 1;
        }
        eo[11] = sum % 2;
        CubieCube {
            co: [0; 8],
            cp: [0; 8],
            eo,
            ep: [0; 12],
        }
    }
}

impl Coordinate for EPCoord {
    type Generator = Htm;
    // 12!/2
    const SIZE: usize = 239500800;

    fn from_cubie_cube(cube: &CubieCube) -> Self {
        let mut ep: u32 = 0;
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        for index in (0..12).rev() {
            let pos = v.iter().position(|&r| r == cube.ep[index] as u16).unwrap() as u32;
            ep += pos;
            v.remove(pos as usize);
            ep *= (index + 1) as u32;
        }
        EPCoord(ep)
    }

    fn to_cubie_cube(&self) -> CubieCube {
        let mut ep: [u8; 12] = [0; 12];
        let mut epcord = self.0;
        let mut v: Vec<usize> = (0..12).collect();
        for index in (0..12).rev() {
            ep[index] = v.remove(epcord as usize % (index + 1)) as u8;
            epcord /= (index + 1) as u32;
        }

        CubieCube {
            co: [0; 8],
            cp: [0; 8],
            eo: [0; 12],
            ep,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CoordCube {
    pub co: COCoord,
    pub cp: CPCoord,
    pub eo: EOCoord,
    pub ep: EPCoord,
}

impl CoordCube {
    pub fn to_cubie(&self) -> CubieCube {
        let co: [u8; 8] = self.co.to_cubie_cube().co;
        let cp: [u8; 8] = self.cp.to_cubie_cube().cp;
        let eo: [u8; 12] = self.eo.to_cubie_cube().eo;
        let ep: [u8; 12] = self.ep.to_cubie_cube().ep;
        CubieCube { co, cp, eo, ep }
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
        let co = COCoord::from_cubie_cube(self);
        let cp = CPCoord::from_cubie_cube(self);
        let eo = EOCoord::from_cubie_cube(self);
        let ep = EPCoord::from_cubie_cube(self);

        CoordCube { co, cp, eo, ep }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invert_identity() {
        let identity = CoordCube::solved();
        assert_eq!(identity, identity.to_cubie().to_coord())
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
