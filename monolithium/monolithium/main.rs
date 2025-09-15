#![allow(dead_code)]
use monolithium::*;
use monolithium::commands::*;

#[derive(Parser)]
#[command(name="monolithium")]
#[command(about="Finding the Largest Minecraft Infdev/Alpha Monoliths")]
enum Commands {
    /// Search for worlds with monoliths near spawn
    Spawn(SpawnCommand),
    /// Find all monoliths in a specific world
    Find(FindCommand),
    /// Make an image of a world's monoliths
    Mask(Mask),
    /// Make an image of a world's perlin noise
    Perlin(PerlinPng),
}

impl Commands {
    fn run(&self) {
        match self {
            Commands::Mask(cmd)   => cmd.run(),
            Commands::Spawn(cmd)  => cmd.run(),
            Commands::Find(cmd)   => cmd.run(),
            Commands::Perlin(cmd) => cmd.run(),
        }
    }
}

fn main() {
    #[cfg(feature="skip-table")]
    JavaRNG::init_skip_table();
    Commands::parse().run();
}
