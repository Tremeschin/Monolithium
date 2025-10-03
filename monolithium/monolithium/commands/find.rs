use crate::*;

#[derive(clap::Args)]
pub struct FindCommand {

    /// World seed to search monoliths
    #[arg(short='s', long, default_value_t=0)]
    seed: u64,

    /// Probe the world every N blocks
    #[arg(short='x', long, default_value_t=128)]
    step: usize,

    /// Minimum area of the monoliths to find
    #[arg(short='a', long, default_value_t=0)]
    area: u64,
}

impl FindCommand {
    pub fn run(&self) {
        let mut world = World::new();
        world.init(self.seed);

        let mut monoliths = world.find_monoliths(
            &FindOptions::default()
                .step(self.step)
                .depth_wraps()
                .threaded()
        );

        monoliths.sort();
        monoliths.iter().for_each(|x| println!("json {}", serde_json::to_string(&x).unwrap()));
        println!("Found {} Monoliths, remember they repeat every {} blocks on any direction!",
            monoliths.len(), MONOLITHS_REPEAT);
    }
}
