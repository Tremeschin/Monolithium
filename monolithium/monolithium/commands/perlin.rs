use crate::*;

#[derive(clap::Args)]
pub struct PerlinPng {
    #[arg(short='s', long, default_value_t=617)]
    seed: Seed,

    #[arg(long, default_value_t=-2000)]
    minx: i32,
    #[arg(long, default_value_t= 2000)]
    maxx: i32,

    #[arg(long, default_value_t=-2000)]
    minz: i32,
    #[arg(long, default_value_t= 2000)]
    maxz: i32,

    #[arg(short='w', long, default_value_t=2048)]
    size: u32,
}

impl PerlinPng {
    pub fn run(&self) {
        let mut world = World::new();
        world.init(self.seed);

        let mut pixels = vec![0u8; (self.size * self.size) as usize];

        for x in 0..self.size {
            for z in 0..self.size {
                let index = (x + z * self.size) as usize;
                let world_x = utils::lerp(
                    (x as f64) / (self.size as f64),
                    self.minx as f64,
                    self.maxx as f64,
                );
                let world_z = utils::lerp(
                    (z as f64) / (self.size as f64),
                    self.minz as f64,
                    self.maxz as f64,
                );
                let value = world.hill.sample(world_x, world_z).abs();
                let pixel = ((value / world.hill.maxval()) * 255.0) as u8;
                pixels[index] = pixel;
            }
        }

        png::Encoder::new(std::fs::File::create("perlin.png").unwrap(), self.size, self.size)
            .write_header().unwrap()
            .write_image_data(&pixels).unwrap();
    }
}