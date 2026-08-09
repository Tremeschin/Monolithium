use crate::*;

#[derive(clap::Subcommand)]
pub enum SeedFactory {

    /// Search in a specific seed value
    Seed {
        #[arg(short='v', long, default_value_t=0)]
        value: Seed,
    },

    /// Search in N sequential seeds from a starting point
    Linear {
        #[arg(short='s', long, default_value_t=0)]
        start: Seed,

        #[arg(short='t', long, default_value_t=1_000_000)]
        total: u64,
    },

    /// Search in N random seeds (fast, duplicates allowed)
    #[command(name="random")]
    FastRandom {
        #[arg(short='t', long, default_value_t=1_000_000)]
        total: u64,
    },

    /// Search in N random seeds (unique, fast enough)
    #[command(name="urandom")]
    UniqueRandom {
        #[arg(short='t', long, default_value_t=1_000_000)]
        total: u64,

        /// Seed for the shared random generator
        #[arg(short='s', long)]
        seed: Option<Seed>,

        #[arg(skip)]
        rng: Arc<Mutex<JavaRNG>>,
    },

    /// Search in a fraction of all possible seeds
    Ratio {
        /// Percentage of all seeds to search (0.0-1.0)
        #[arg(short='r', long, default_value_t=1.0)]
        ratio: f64,
    },

    /// Search in a file with a list of seeds or monoliths
    File {
        #[arg(short='i', long)]
        input: String,
        values: Vec<Seed>,
    }
}


impl SeedFactory {
    pub fn initialize(&mut self) {
        match self {
            Self::File{input, values} => {
                let content = std::fs::read_to_string(input)
                    .expect("Could not read input file");

                for line in content.lines() {
                    let line = line.trim();

                    // From a piped monoliths json
                    if line.starts_with('{') {
                        let monolith = serde_json::from_str::<Monolith>(line)
                            .expect("Could not parse Monolith from JSON");
                        values.push(monolith.seed);

                    // Try parsing as number
                    } else if let Ok(seed) = line.parse::<Seed>() {
                        values.push(seed);
                    }
                }
            },

            Self::UniqueRandom { seed, rng, .. } => {
                let seed = seed.unwrap_or_else(|| fastrand::u64(0..TOTAL_SEEDS));
                *rng = Arc::new(Mutex::new(JavaRNG::from_state(seed)));
            },

            _ => ()
        }
    }

    pub fn total(&self) -> Seed {
        match self {
            Self::Seed{..} => 1,
            Self::Linear{total, ..} => *total,
            Self::FastRandom{total, ..} => *total,
            Self::UniqueRandom{total, ..} => *total,
            Self::Ratio{ratio} => (ratio * TOTAL_SEEDS as f64) as Seed,
            Self::File{values, ..} => values.len() as Seed,
        }
    }

    #[inline(always)]
    pub fn get(&self, n: u64) -> Seed {
        match self {
            Self::Seed{value} =>
                *value,

            Self::Linear{start, ..} =>
                (*start + n) as Seed,

            Self::FastRandom{..} =>
                fastrand::u64(0..TOTAL_SEEDS),

            Self::UniqueRandom { rng, .. } => {
                let mut rng = rng.lock().unwrap();
                rng.step();
                rng.state as Seed
            }

            Self::Ratio{ratio} =>
                (n as f64 / *ratio) as Seed,

            Self::File{values, ..} =>
                values[n as usize],
        }
    }
}
