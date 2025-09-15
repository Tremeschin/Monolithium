use crate::*;

#[derive(clap::Args)]
pub struct SpawnCommand {

    #[command(subcommand)]
    seeds: SeedFactory,

    /// How many seeds each work block should process
    #[arg(short='c', long, default_value_t=1)]
    chunks: u64,

    /// How far from spawn to search in a square radius
    #[arg(short='r', long, default_value_t=100)]
    radius: i64,

    /// Spacing between each check, in blocks
    #[arg(short='s', long, default_value_t=200)]
    step: usize,
}

impl SpawnCommand {
    pub fn run(&self) {

        // Standard math to split a work into many blocks
        let chunks = (self.seeds.total() + self.chunks - 1) / self.chunks;

        let progress = ProgressBar::new(chunks)
            .with_style(utils::progress("Searching"));

        let options = FindOptions::default()
            .step(self.step)
            .spawn(self.radius)
            .limit(1);

        let mut monoliths: Vec<Monolith> =
            (0..=chunks)
            .into_par_iter()
            .progress_with(progress)
            .map_init(|| World::new(), |world, chunk| {
                let c_a = (chunk + 0) * self.chunks;
                let c_b = (chunk + 1) * self.chunks;

                (c_a..c_b).map(|seed| {
                    world.init(self.seeds.get(seed));
                    world.find_monoliths(&options)
                }).flatten()
                  .collect::<Vec<Monolith>>()
            })
            .flatten()
            .collect();

        monoliths.sort();
        monoliths.iter().for_each(|x| println!("json {}", serde_json::to_string(&x).unwrap()));
        println!("Found {} Monoliths", monoliths.len());
    }
}
