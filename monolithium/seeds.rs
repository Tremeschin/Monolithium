use crate::*;

// Fixme: IntoParallelIterator without Vec<u64>
// with progressive yielding to fix memory hog

#[derive(clap::Subcommand)]
pub enum SeedFactory {

    /// Search in N sequential seeds from a starting point
    Linear {
        #[arg(short='s', long, default_value_t=0)]
        start: u64,

        #[arg(short='c', long, default_value_t=1_000_000)]
        count: u64,
    },

    // Search in N unique random seeds
    Random {
        #[arg(short='n', long, default_value_t=1_000_000)]
        total: usize,
    },
}


impl SeedFactory {
    pub fn values(&self) -> Vec<u64> {
        match self {
            SeedFactory::Linear{start, count} =>
                (*start.. (*start + *count)).collect(),

            SeedFactory::Random{total} => {
                let mut set = HashSet::with_capacity(*total);
                while set.len() < *total {
                    set.insert(rand::random_range(0..TOTAL_SEEDS));
                }
                set.into_iter().collect()
            }
        }
    }
}
