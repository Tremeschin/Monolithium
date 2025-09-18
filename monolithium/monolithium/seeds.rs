use crate::*;

#[derive(clap::Subcommand)]
pub enum SeedFactory {

    /// Search in a specific given seed
    Seed {
        #[arg(short='s', long, default_value_t=0)]
        value: u64,
    },

    /// Search in N sequential seeds from a starting point
    Linear {
        #[arg(short='s', long, default_value_t=0)]
        start: u64,

        #[arg(short='t', long, default_value_t=1_000_000)]
        total: u64,
    },

    /// Search in N random seeds
    Random {
        #[arg(short='t', long, default_value_t=1_000_000)]
        total: u64,
    },

    /// Search in a fraction of all possible seeds
    Ratio {
        /// Percentage of all seeds to search (0.0-1.0)
        #[arg(short='r', long, default_value_t=1.0)]
        ratio: f64,
    }
}


impl SeedFactory {
    pub fn total(&self) -> u64 {
        match self {
            Self::Seed{..} => 1,
            Self::Linear{total, ..} => *total,
            Self::Random{total, ..} => *total,
            Self::Ratio{ratio} => (ratio * TOTAL_SEEDS as f64) as u64,
        }
    }

    pub fn get(&self, n: u64) -> u64 {
        match self {
            Self::Seed{value} =>
                *value,

            Self::Linear{start, ..} =>
                (*start + n) as u64,

            // Fixme: Birthday paradox N = 2**48
            Self::Random{..} =>
                fastrand::u64(0..TOTAL_SEEDS),

            Self::Ratio{ratio} =>
                (n as f64 / *ratio) as u64,
        }
    }
}
