use crate::*;

#[derive(Clone, Eq)]
pub struct Monolith {
    // Position in the world
    pub minx: i64, pub maxx: i64,
    pub minz: i64, pub maxz: i64,

    pub area: u64,
    pub seed: u64,
}

impl Monolith {
    pub fn center_x(&self) -> i64 {
        (self.minx + self.maxx) / 2
    }

    pub fn center_z(&self) -> i64 {
        (self.minz + self.maxz) / 2
    }
}

/* -------------------------------------------------------------------------- */
// Monoliths are equal if they have the same coordinates

impl Hash for Monolith {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.minx.hash(state);
        self.minz.hash(state);
    }
}

impl PartialEq for Monolith {
    fn eq(&self, other: &Self) -> bool {
        (self.minx == other.minx) && (self.minz == other.minz)
    }
}

/* -------------------------------------------------------------------------- */
// Monoliths should be sorted by area

impl PartialOrd for Monolith {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.area.cmp(&other.area))
    }
}

impl Ord for Monolith {
    fn cmp(&self, other: &Self) -> Ordering {
        self.area.cmp(&other.area)
    }
}
