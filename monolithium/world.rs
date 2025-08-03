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

    pub fn find_monoliths(&self, query: &FindOptions) -> Vec<Monolith> {
        let xrange: Vec<i64> = (query.minx..=query.maxx).step_by(query.spacing).collect();
        let zrange: Vec<i64> = (query.minz..=query.maxz).step_by(query.spacing).collect();

        // Use non-threaded approach for small areas (lower latency)
        if (query.maxx - query.minx).abs() < 1000 {
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
            let progress = ProgressBar::new(xrange.len() as u64).with_style(
                ProgressStyle::default_bar()
                    .template("Searching ({elapsed_precise} • ETA {eta}) {wide_bar:.cyan/blue} ({percent}%) • {pos}/{len}")
                    .unwrap()
            );

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

    pub fn spacing(&mut self, spacing: usize) -> &mut Self {
        self.spacing = spacing;
        return self;
    }

    pub fn limit(&mut self, many: u64) -> &mut Self {
        self.limit = Some(many);
        return self;
    }

    // Defining regions

    /// Search around a given coordinate at most `radius` manhattan blocks away
    pub fn around(&mut self, x: i64, z: i64, radius: i64) -> &mut Self {
        self.minx = x - radius;
        self.maxx = x + radius;
        self.minz = z - radius;
        self.maxz = z + radius;
        return self;
    }

    /// Search around spawn at most `radius` manhattan blocks away
    pub fn spawn(&mut self, radius: i64) -> &mut Self {
        self.around(0, 0, radius)
    }

    /// Search all blocks before the Far Lands
    pub fn inbounds(&mut self) -> &mut Self {
        self.minx = -FARLANDS;
        self.maxx =  FARLANDS;
        self.minz = -FARLANDS;
        self.maxz =  FARLANDS;
        return self;
    }

    pub fn wraps(&mut self) -> &mut Self {
        self.minx = 0;
        self.maxx = WORLD_WRAP;
        self.minz = 0;
        self.maxz = WORLD_WRAP;
        return self;
    }
}
