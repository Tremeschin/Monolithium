#![allow(dead_code)]
use monolithium::*;
use monolithium::commands::*;

#[derive(Parser)]
#[command(name="monolithium")]
#[command(about="Finding the Largest Minecraft Alpha Monoliths")]
enum Commands {
    /// Search for worlds with monoliths near spawn
    Search(SearchCommand),
    /// Make an image of a world's monoliths
    Mask(Mask),
    /// Make an image of a world's perlin noise
    Perlin(PerlinPng),
}

impl Commands {
    fn run(&mut self) {
        match self {
            Commands::Mask(cmd)   => cmd.run(),
            Commands::Search(cmd) => cmd.run(),
            Commands::Perlin(cmd) => cmd.run(),
        }
    }
}

fn main() {
    Commands::parse().run();
}
