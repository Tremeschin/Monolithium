use crate::*;

pub struct World {
    pub seed:  u64,
    pub hill:  FractalPerlin<10>,
    pub depth: FractalPerlin<16>,
}

impl World {
    pub fn new() -> Self {
        World {
            seed: 0,
            hill:  FractalPerlin::new(),
            depth: FractalPerlin::new(),
        }
    }

    pub fn init(&mut self, seed: u64) {
        let mut rng = JavaRNG::new(seed);

        // Skip 48 generators priorly used elsewhere
        PerlinNoise::discard(&mut rng, 48);

        self.hill.init(&mut rng);
        self.depth.init(&mut rng);
        self.seed = seed;
    }

    // Check if a given coordinate is part of a monolith
    pub fn is_monolith(&self, x: i64, z: i64) -> bool {
        self.depth.sample(
            ((x/4) as f64) * 100.0,
            ((z/4) as f64) * 100.0,
        ).abs() > 8000.0
        &&
        self.hill.sample(
            (x/4) as f64,
            (z/4) as f64,
        ) < -512.0
    }

    /// Get a Monolith at a given coordinate, compute properties
    /// Todo: Arc Mutex HashMap (x, y) => Monolith struct?
    pub fn get_monolith(&self, x: i64, z: i64) -> Option<Monolith> {

        // Most blocks are not monoliths
        if !self.is_monolith(x, z) {
            return None;
        }

        let x = utils::nearest(x, 4);
        let z = utils::nearest(z, 4);
        let o = 32; // "Occasionally"

        // Start with current block
        let mut lith = Monolith {
            minx: (x+o), minz: (z+o),
            maxx: (x-o), maxz: (z-o),
            seed: self.seed,
            area: 0,
        };

        // Using a Breadth First Search like approach
        let mut visited = AHashSet::from([(x, z)]);
        let mut queue   = VecDeque::from([(x, z)]);

        // Search around the block
        let far: i64 = 256;
        for dx in (-far..=far).step_by(16) {
            for dz in (-far..=far).step_by(16) {
                if (dx*dx + dz*dz) < far*far {
                    queue.push_back((x+dx, z+dz));
                }
            }
        }

        while let Some((x, z)) = queue.pop_front() {
            if !self.is_monolith(x, z) {
                continue;
            }

            lith.area += 16;

            // Check neighbors with step 4 per hill/depth scaling
            let mut neighbors = vec!((0, 4), (4, 0), (0, -4), (-4, 0));

            // Occasional more expensive stuff
            if (x % o == 0) && (z % o == 0) {

                // Check for nearby disjoints
                for factor in [1, 4] {
                    let n = 64*factor;
                    neighbors.extend(vec!(
                        ( n,  n), ( n, -n),
                        (-n,  n), (-n, -n),
                        ( n,  0), ( 0,  n),
                        (-n,  0), ( 0, -n),
                    ))
                }

                // Update coordinates
                lith.minx = min(lith.minx, x);
                lith.maxx = max(lith.maxx, x);
                lith.minz = min(lith.minz, z);
                lith.maxz = max(lith.maxz, z);
            }

            for (dx, dz) in neighbors {
                if visited.insert((x, z)) {
                    queue.push_back((x+dx, z+dz));
                }
            }
        }

        Some(lith)
    }

    pub fn find_monoliths(&self, query: &FindOptions) -> Vec<Monolith> {
        let xrange: Vec<i64> = (query.minx..=query.maxx).step_by(query.spacing).collect();
        let zrange: Vec<i64> = (query.minz..=query.maxz).step_by(query.spacing).collect();

        // Use non-threaded approach for small areas (lower latency)
        if (query.maxx - query.minx).abs() < 10000 {
            let mut monoliths = AHashSet::new();

            'a: for x in xrange.clone() {
                for z in zrange.clone() {
                    if let Some(mono) = self.get_monolith(x, z) {
                        monoliths.insert(mono);

                        // Early break if limit is reached
                        if let Some(many) = query.limit {
                            if monoliths.len() >= many as usize {
                                break 'a;
                            }
                        }
                    }
                }
            }
            return monoliths
                .into_iter().collect();

        // Shred the cpu.
        // Why bother breaking after a limit?
        } else {
            let monoliths = Arc::new(Mutex::new(AHashSet::new()));

            // Nice to have an estimative of the progress yknow..
            let progress = ProgressBar::new(xrange.len() as u64)
                .with_style(utils::progress("Searching"));

            xrange.clone()
                .into_par_iter()
                .progress_with(progress)
                .for_each(|x| {
                    for z in zrange.clone() {
                        if let Some(mono) = self.get_monolith(x, z) {
                            let mut monoliths = monoliths.lock().unwrap();
                            monoliths.insert(mono);
                        }
                    }
                });

            return monoliths
                .lock().unwrap().clone()
                .into_iter().collect();
        }
    }
}

/* -------------------------------------------------------------------------- */

#[derive(SmartDefault)]
pub struct FindOptions {
    pub minx: i64,
    pub maxx: i64,
    pub minz: i64,
    pub maxz: i64,

    #[default(32)]
    pub spacing: usize,

    /// How many monoliths to find
    pub limit: Option<u64>,
}

impl FindOptions {

    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        return self;
    }

    pub fn limit(mut self, many: u64) -> Self {
        self.limit = Some(many);
        return self;
    }

    // Defining regions

    /// Search around a given coordinate at most `radius` manhattan blocks away
    pub fn around(mut self, x: i64, z: i64, radius: i64) -> Self {
        self.minx = x - radius;
        self.maxx = x + radius;
        self.minz = z - radius;
        self.maxz = z + radius;
        return self;
    }

    /// Search around spawn at most `radius` manhattan blocks away
    pub fn spawn(self, radius: i64) -> Self {
        self.around(0, 0, radius)
    }

    /// Search all blocks before the Far Lands
    pub fn inbounds(mut self) -> Self {
        self.minx = -FARLANDS;
        self.maxx =  FARLANDS;
        self.minz = -FARLANDS;
        self.maxz =  FARLANDS;
        return self;
    }

    pub fn wraps(mut self) -> Self {
        self.minx = 0;
        self.maxx = PERLIN_WRAP;
        self.minz = 0;
        self.maxz = PERLIN_WRAP;
        return self;
    }
}
