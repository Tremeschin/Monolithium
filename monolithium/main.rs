#![allow(dead_code)]
use monolithium::*;

fn world_png() {
    let world = World::new(2527251);
    let (minx, minz): (i64, i64) = (-2000, -2000);
    let (maxx, maxz): (i64, i64) = ( 2000,  2000);
    let width  = ((maxx - minx) as u32)/4;
    let height = ((maxz - minz) as u32)/4;

    let mut pixels = vec![0u8; (width * height) as usize];
    {
        let mut index = 0;
        for x in (minx..maxx).step_by(4) {
            for z in (minz..maxz).step_by(4) {
                if world.is_monolith(x as i64, z as i64, true) {
                    pixels[index as usize] = 255;
                }
                if (x % 250 == 0) || (z % 250 == 0) {
                    pixels[index as usize] = 64;
                }
                index += 1;
            }
        }
    }

    png::Encoder::new(std::fs::File::create("monoliths.png").unwrap(), width, height)
        .write_header().unwrap()
        .write_image_data(&pixels).unwrap();
}

fn biggest_spawn_monoliths() {
    let _seeds = SeedFactory::Linear{start: 0, count: 1_000_000_000}.values();
    let _seeds = SeedFactory::Random{total: 1_000_000}.values();

    let progress = ProgressBar::new(_seeds.len() as u64)
        .with_style(utils::progress("Searching"));

    // Iterate seeds to find monoliths near spawn
    let mut monoliths: Vec<Monolith> =
        _seeds
        .into_par_iter()
        .progress_with(progress)
        .map(|seed| {
            let world = World::new(seed);
            let monoliths = world.find_monoliths(
                &FindOptions::default()
                    .spawn(202).spacing(200).limit(1)
            );
            monoliths
        }).flatten()
        .collect();

    monoliths.sort();
    monoliths.iter().for_each(|x| println!("{:?}", x));
}

fn whole_world_monoliths() {
    let world = World::new(617);

    let mut monoliths = world.find_monoliths(
        // &FindOptions::default().wraps().spacing(256));
        &FindOptions::default().spawn(500).spacing(4));

    monoliths.sort();
    monoliths.iter().for_each(|x| println!("{:?}", x));
    println!("Found {} Monoliths", monoliths.len());
}

fn benchmark() {
    use microbench::Options;
    use std::hint::black_box;

    let world = World::new(617);

    let options = Options::default();
    microbench::bench(&options, "is_monolith", || {
        for x in -1000..1000 {
            for z in -1000..1000 {
                black_box(world.is_monolith(x, z, true));
            }
        }
    });
}


fn perlinpng() {
    let world  = World::new(617);
    let repeat = world.hill.repeats() as f64;

    let (width, height) = (2048, 2048);
    let (min_x, min_z)  = (-repeat, -repeat);
    let (max_x, max_z)  = ( repeat,  repeat);

    let mut pixels = vec![0u8; (width * height) as usize];

    for x in 0..width {
        for z in 0..height {
            let index = (x + z * width) as usize;
            let x = utils::lerp(x as f64 / width  as f64, min_x, max_x);
            let z = utils::lerp(z as f64 / height as f64, min_z, max_z);
            let value = world.hill.sample(x, 0.0, z, true).abs();

            let pixel = ((value / world.hill.maxval()) * 255.0) as u8;
            pixels[index] = pixel;
        }
    }

    png::Encoder::new(std::fs::File::create("perlin.png").unwrap(), width, height)
        .write_header().unwrap()
        .write_image_data(&pixels).unwrap();
}

fn main() {
    // world_png();
    // benchmark();
    biggest_spawn_monoliths();
    // whole_world_monoliths();
    // find_low_entropy_seeds();
    // perlinpng();
}