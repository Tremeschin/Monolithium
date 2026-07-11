use crate::*;

#[derive(clap::Args)]
pub struct SearchCommand {

    #[command(subcommand)]
    pub seeds: SeedFactory,

    /// (Worker ) How many seeds each work block should process
    #[arg(short='c', long, default_value_t=1)]
    pub chunks: u64,

    /// (Worker ) Use multithreading to search within a seed
    #[arg(short='t', long, default_value_t=false)]
    pub threaded: bool,

    /// (Worker ) Completely disable log messages while running. Meant for
    /// benchmarking
    #[arg(long, default_value_t = false)]
    pub silent: bool,

    /// (Where  ) Center X value to search for monoliths
    #[arg(short='x', long, default_value_t=0)]
    pub center_x: i32,

    /// (Where  ) Center Z value to search for monoliths
    #[arg(short='z', long, default_value_t=0)]
    pub center_z: i32,

    /// (Where  ) How far from spawn to search in a square radius
    #[arg(short='r', long, default_value_t=100)]
    pub radius: i32,

    /// (Where  ) Spacing between each check, in blocks
    #[arg(short='s', long, default_value_t=200)]
    pub step: usize,

    /// (Limits ) Maximum number of monoliths to find in a seed
    #[arg(short='l', long, default_value_t=999999)]
    pub limit: u64,

    /// (Limits ) Minimum area of the monoliths to find
    #[arg(short='a', long, default_value_t=0)]
    pub area: u64,

    /// (Special) Set radius to the value hill noise wraps (262144)
    #[arg(short='h', long, default_value_t=false)]
    pub hill: bool,

    /// (Special) Set radius to the value depth noise wraps (4194304)
    #[arg(short='d', long, default_value_t=false)]
    pub depth: bool,
}

impl SearchCommand {
    pub fn run(&mut self) {
        self.seeds.initialize();

        // Standard math to split a work into many blocks
        let chunks = (self.seeds.total() + self.chunks - 1) / self.chunks;

        let progress = if !self.silent {
            ProgressBar::new(chunks).with_style(utils::progress("Searching"))
        } else {
            ProgressBar::hidden()
        };

        let mut options = FindOptions::default()
            .around(self.center_x, self.center_z, self.radius)
            .threaded(self.threaded)
            .limit(self.limit)
            .area(self.area)
            .step(self.step);

        // Apply sugar options
        if self.hill  {options = options.hill_wraps(); }
        if self.depth {options = options.depth_wraps();}

        // Infer threading if too few inputs
        if self.seeds.total() < 4 {
            options = options.threaded(true);
        }

        let mut monoliths: Vec<Monolith> =
            (0..chunks)
            .into_par_iter()
            .progress_with(progress)
            .map_init(|| World::new(), |world, chunk| {
                let min = (chunk + 0) * self.chunks;
                let max = (chunk + 1) * self.chunks;

                if cfg!(feature="sister-perlin") {
                    let seed = self.seeds.get(min);
                    world.init(seed);
                }

                (min..max).map(|seed| {
                    if cfg!(feature="sister-perlin") {
                        world.sister_perlin();
                        world.find_monoliths(&options)
                    } else {
                        let seed = self.seeds.get(seed);

                        #[cfg(feature="filter-fracts")]
                        if !World::good_perlin_fracts(seed) {
                            return Vec::new();
                        }

                        world.init(seed);
                        world.find_monoliths(&options)
                    }
                }).flatten()
                  .collect::<Vec<Monolith>>()
            })
            .flatten()
            .collect();

        monoliths.sort();
        if !self.silent {
            monoliths.iter().for_each(|x| println!("{}", serde_json::to_string(&x).unwrap()));
            println!("Found {} Monoliths", monoliths.len());
        }
    }
}
