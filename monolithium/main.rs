#![allow(dead_code)]
use monolithium::*;

fn world_png() {
    let world = World::new(8306587);
    let (minx, minz): (i64, i64) = (-1000, -1000);
    let (maxx, maxz): (i64, i64) = ( 1000,  1000);
    let width  = ((maxx - minx) as u32)/4;
    let height = ((maxz - minz) as u32)/4;

    let mut pixels = vec![0u8; (width * height) as usize];
    {
        let mut index = 0;
        for x in (minx..maxx).step_by(4) {
            println!("Processing x = {}", x);
            for z in (minz..maxz).step_by(4) {
                if world.is_monolith(x as i64, z as i64) {
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

    // Iterate seeds to find monoliths near spawn
    let mut monoliths: Vec<Monolith> =
        (0..=1_000_000)
        .into_par_iter()
        .map(|seed| {
            let world = World::new(seed);
            let monoliths = world.find_monoliths(
                &FindOptions::default()
                    .spawn(200).spacing(64).limit(1)
            );
            monoliths
        }).flatten()
        .collect();

    monoliths.sort();

    for mono in monoliths {
        println!("Monolith (Area: {:>7}) at ({:>5}, {:>5}) with seed {}",
            mono.area, mono.center_x(), mono.center_z(), mono.seed);
    }
}

fn whole_world_monoliths() {
    let world = World::new(851328);

    let mut monoliths = world.find_monoliths(
        &FindOptions::default().wraps().spacing(256));

    monoliths.sort();

    for mono in &monoliths {
        println!("Monolith (Area: {:>7}) at ({:>5}, {:>5}) with seed {}",
            mono.area, mono.center_x(), mono.center_z(), mono.seed);
    }

    println!("Found {} monoliths", monoliths.len());
}

fn benchmark() {
    use microbench::Options;
    use std::hint::black_box;

    let world = World::new(617);

    let options = Options::default();
    microbench::bench(&options, "is_monolith", || {
        for x in -1000..1000 {
            for z in -1000..1000 {
                black_box(world.is_monolith(x, z));
            }
        }
    });
}

fn main() {
    // world_png();
    // benchmark();
    // biggest_spawn_monoliths();
    whole_world_monoliths();
    // find_low_entropy_seeds();
}