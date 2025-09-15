use crate::*;

#[derive(clap::Args)]
pub struct FindCommand {

    /// World seed to search monoliths
    #[arg(short='s', long, default_value_t=0)]
    seed: u64,

    /// Probe the world every N blocks
    #[arg(short='x', long, default_value_t=128)]
    spacing: usize,
}

impl FindCommand {
    pub fn run(&self) {
        let mut world = World::new();
        world.init(self.seed);

        let mut monoliths = world.find_monoliths(
            &FindOptions::default()
                .wraps().spacing(self.spacing)
        );

        monoliths.sort();
        monoliths.iter().for_each(|x| println!("{:?}", x));
        println!("Found {} Monoliths, remember they repeat every {} blocks on any direction!",
            monoliths.len(), MONOLITHS_REPEAT);
    }
}
