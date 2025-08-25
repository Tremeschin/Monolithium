use crate::*;

#[derive(clap::Args)]
pub struct SpawnCommand {

    #[command(subcommand)]
    seeds: SeedFactory,

    #[arg(short='r', long, default_value_t=100)]
    radius: i64,

    #[arg(short='s', long, default_value_t=200)]
    spacing: usize,
}

impl SpawnCommand {
    pub fn run(&self) {
        let progress = ProgressBar::new(self.seeds.total())
            .with_style(utils::progress("Searching"));

        let options = FindOptions::default()
            .spacing(self.spacing)
            .spawn(self.radius)
            .limit(1);

        let mut monoliths: Vec<Monolith> =
            (0..=self.seeds.total())
            .into_par_iter()
            .progress_with(progress)
            .map_init(|| World::new(), |world, seed| {
                world.init(self.seeds.get(seed));
                world.find_monoliths(&options)
            })
            .flatten()
            .collect();

        monoliths.sort();
        monoliths.iter().for_each(|x| println!("{:?}", x));
        println!("Found {} Monoliths", monoliths.len());
    }
}
