use crate::*;

#[derive(clap::Args)]
pub struct Mask {
    #[arg(long, default_value_t=617)]
    seed: Seed,

    #[arg(long, default_value_t=-2000)]
    minx: i32,
    #[arg(long, default_value_t= 2000)]
    maxx: i32,

    #[arg(long, default_value_t=-2000)]
    minz: i32,
    #[arg(long, default_value_t= 2000)]
    maxz: i32,
}

impl Mask {
    pub fn run(&self) {
        let mut world  = World::new();
        world.init(self.seed);

        let width  = ((self.maxx - self.minx) as u32) / 4;
        let height = ((self.maxz - self.minz) as u32) / 4;
        let mut pixels = vec![0u8; (width * height) as usize];

        let mut index = 0;
        for x in (self.minx..self.maxx).step_by(4) {
            for z in (self.minz..self.maxz).step_by(4) {
                if world.is_monolith(x, z) {
                    pixels[index] = 255;
                }
                if (x % 250 == 0) || (z % 250 == 0) {
                    pixels[index] = 64;
                }
                index += 1;
            }
        }

        png::Encoder::new(std::fs::File::create("monoliths.png").unwrap(), width, height)
            .write_header().unwrap()
            .write_image_data(&pixels).unwrap();
    }
}
