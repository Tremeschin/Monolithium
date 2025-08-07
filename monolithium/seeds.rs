use crate::*;

// Fixme: IntoParallelIterator without Vec<u64>
// with progressive yielding to fix memory hog
pub enum SeedFactory {
    Linear {start: u64, count: u64},
    Random {total: usize},
}

impl SeedFactory {
    pub fn values(self) -> Vec<u64> {
        match self {
            SeedFactory::Linear{start, count} =>
                (start..start+count).collect(),

            SeedFactory::Random{total} => {
                let mut set = HashSet::with_capacity(total);
                while set.len() < total {
                    set.insert(rand::random_range(0..TOTAL_SEEDS));
                }
                set.into_iter().collect()
            }
        }
    }
}
