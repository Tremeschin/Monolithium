use crate::*;

pub struct World {
    pub seed:  u64,
    pub hill:  FractalPerlin<10>,
    pub depth: FractalPerlin<16>,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let mut rng = Random::new(seed);

        // Skip 48 generators priorly used elsewhere
        for _ in 0..48 {PerlinNoise::new(&mut rng);}

        World {
            seed:  seed,
            hill:  FractalPerlin::new(&mut rng),
            depth: FractalPerlin::new(&mut rng),
        }
    }

    // Check if a given coordinate is part of a monolith
    pub fn is_monolith(&self, x: i64, z: i64) -> bool {
        self.depth.sample(
            ((x/4) as f64) * 100.0, 0.0,
            ((z/4) as f64) * 100.0
        ).abs() > 8000.0
        &&
        self.hill.sample(
            ((x/4) as f64) * 1.0, 0.0,
            ((z/4) as f64) * 1.0
        ) < -512.0
    }

    /// Get a Monolith at a given coordinate, compute properties
    pub fn get_monolith(&self, x: i64, z: i64) -> Option<Monolith> {
        let x = utils::nearest(x, 4);
        let z = utils::nearest(z, 4);

        if !self.is_monolith(x, z) {
            return None;
        }

        // Start with current block
        let mut mono = Monolith {
            minx: x, minz: z,
            maxx: x, maxz: z,
            seed: self.seed,
            area: 0,
        };

        // Using a Breadth First Search like approach
        let mut visited = AHashSet::from([(x, z)]);
        let mut queue   = VecDeque::from([(x, z)]);

        while let Some((x, z)) = queue.pop_front() {
            mono.minx = min(mono.minx, x);
            mono.maxx = max(mono.maxx, x);
            mono.minz = min(mono.minz, z);
            mono.maxz = max(mono.maxz, z);
            mono.area += 16;

            // Check neighbors with step 4 per hill/depth scaling
            let mut neighbors = vec!((0, 4), (4, 0), (0, -4), (-4, 0));

            // Occasionally check for disjoints
            if (x % 32 == 0) && (z % 32 == 0) {
                let n = 64;
                neighbors.extend(vec!(
                    ( n,  n), ( n, -n),
                    (-n,  n), (-n, -n),
                ))
            }

            for (dx, dz) in neighbors {
                let (nx, nz) = (x+dx, z+dz);

                if visited.contains(&(nx, nz)) {
                    continue;
                }

                if self.is_monolith(nx, nz) {
                    visited.insert((nx, nz));
                    queue.push_back((nx, nz));
                }
            }
        }

        Some(mono)
    }

    pub fn find_monoliths(&self,
        minx: i64, minz: i64,
        maxx: i64, maxz: i64,
        spacing: usize
    ) -> Vec<Monolith> {

        // Use non-threaded approach for small areas
        if (maxx - minx) < 1000 {
            let mut monoliths = AHashSet::new();

            for x in (minx..=maxx).step_by(spacing) {
                for z in (minz..=maxz).step_by(spacing) {
                    if let Some(mono) = self.get_monolith(x, z) {
                        monoliths.insert(mono);
                    }
                }
            }
            return monoliths
                .into_iter().collect();

        // Shred the cpu.
        } else {
            let monoliths = Arc::new(Mutex::new(AHashSet::new()));

            (minx..=maxx)
                .step_by(spacing)
                .collect::<Vec<i64>>()
                .into_par_iter()
                .for_each(|x| {
                    for z in (minz..=maxz).step_by(spacing) {
                        if let Some(mono) = self.get_monolith(x, z) {
                            monoliths.lock().unwrap().insert(mono);
                        }
                    }
                });

            return monoliths
                .lock().unwrap().clone()
                .into_iter().collect();
        }
    }
}