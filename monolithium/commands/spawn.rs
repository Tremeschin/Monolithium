use crate::*;

#[derive(clap::Args)]
pub struct SpawnCommand {

    #[command(subcommand)]
    seeds: SeedFactory,

    #[arg(short='r', long, default_value_t=150)]
    radius: i64,

    #[arg(short='s', long, default_value_t=300)]
    spacing: usize,
}

impl SpawnCommand {
    pub fn run(&self) {
        let seeds = self.seeds.values();
        let progress = ProgressBar::new(seeds.len() as u64)
            .with_style(utils::progress("Searching"));

        let mut monoliths: Vec<Monolith> = seeds
            .into_par_iter()
            .progress_with(progress)
            .map(|seed| {
                let world = World::new(seed);
                world.find_monoliths(
                    &FindOptions::default()
                        .spawn(self.radius)
                        .spacing(self.spacing)
                        .limit(1)
                )
            })
            .flatten()
            .collect();

        monoliths.sort();
        monoliths.iter().for_each(|x| println!("{:?}", x));
        println!("Found {} Monoliths", monoliths.len());
    }
}
