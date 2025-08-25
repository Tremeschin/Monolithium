use crate::*;

#[derive(clap::Args)]
pub struct PerlinPng {
    #[arg(short='s', long, default_value_t=617)]
    seed: u64,

    #[arg(short='w', long, default_value_t=2048)]
    size: u32,
}

impl PerlinPng {
    pub fn run(&self) {
        let mut world = World::new();
        world.init(self.seed);

        let repeat = world.hill.repeats() as f64;
        let (min_x, min_z) = (-repeat, -repeat);
        let (max_x, max_z) = ( repeat,  repeat);
        let mut pixels = vec![0u8; (self.size * self.size) as usize];

        for x in 0..self.size {
            for z in 0..self.size {
                let index = (x + z * self.size) as usize;
                let world_x = utils::lerp(x as f64 / self.size as f64, min_x, max_x);
                let world_z = utils::lerp(z as f64 / self.size as f64, min_z, max_z);
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