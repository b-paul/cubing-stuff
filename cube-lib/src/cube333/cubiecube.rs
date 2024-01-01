use super::{CubieCube, CornerTwist, EdgeFlip};

impl CubieCube {
    /// Returns the sum of corner orientations over the cube, represented as a corner orientation.
    pub fn co_parity(&self) -> CornerTwist {
        self.co.into_iter().fold(CornerTwist::Oriented, |acc, ct| acc.twist_by(ct))
    }

    /// Returns the sum of edge orientations over the cube, represented as an edge orientation.
    pub fn eo_parity(&self) -> EdgeFlip {
        self.eo.into_iter().fold(EdgeFlip::Oriented, |acc, ef| acc.flip_by(ef))
    }

    // The parity of a permutation is equal to the sum over each cycle of (elems in cycle - 1).
    fn cp_parity(&self) -> bool {
        let mut parity = false;
        let mut visited = [false; 8];
        for start_corner in self.cp {
            if visited[start_corner as usize] {
                continue;
            }
            let mut ptr = start_corner;
            visited[ptr as usize] = true;
            while self.cp[ptr as usize] != start_corner {
                parity = !parity;
                ptr = self.cp[ptr as usize];
                visited[ptr as usize] = true;
            }
        }
        parity
    }

    fn ep_parity(&self) -> bool {
        let mut parity = false;
        let mut visited = [false; 12];
        for start_edge in self.ep {
            if visited[start_edge as usize] {
                continue;
            }
            let mut ptr = start_edge;
            visited[ptr as usize] = true;
            while self.ep[ptr as usize] != start_edge {
                parity = !parity;
                ptr = self.ep[ptr as usize];
                visited[ptr as usize] = true;
            }
        }
        parity
    }

    /// Return the permutation parity of the cube as a bool
    pub fn perm_parity(&self) -> bool {
        self.ep_parity() ^ self.cp_parity()
    }

    /// Return true if the corresponding cube is illegal due to a corner twist, an edge flip or by
    /// permutation parity.
    pub fn illegal(&self) -> bool {
        self.co_parity() != CornerTwist::Oriented ||
        self.eo_parity() != EdgeFlip::Oriented ||
        self.perm_parity()
    }
}
