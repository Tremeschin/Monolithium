use crate::*;

pub const HILL_OCTAVES:  usize = 10;
pub const DEPTH_OCTAVES: usize = 16;

#[derive(Debug)]
pub struct World {
    pub seed: u64,

    /// Noise which determines how 'flat' the terrain is via elevation, with
    /// values below -512.0 being required to form a monolith.
    ///
    /// - This is the rarest of the two conditions
    ///
    pub hill: FractalPerlin<HILL_OCTAVES>,

    /// Noise which modulates the hill factor's influence, with absolute values
    /// greater than 8000.0 being required to form a monolith.
    ///
    /// - About 40% of all blocks in any world satisfy this condition.
    ///
    #[cfg(not(feature="only-hill"))]
    pub depth: FractalPerlin<DEPTH_OCTAVES>,
}

impl World {
    pub fn new() -> Self {
        World {
            seed: 0,
            hill: FractalPerlin::new(),

            #[cfg(not(feature="only-hill"))]
            depth: FractalPerlin::new(),
        }
    }

    #[inline(always)]
    pub fn init(&mut self, seed: u64) {
        let mut rng = JavaRNG::new(seed);
        self.seed = seed;

        // Skip 48 generators priorly used elsewhere
        Perlin::discard(&mut rng, 48);

        self.hill.init(&mut rng);

        #[cfg(not(feature="only-hill"))]
        self.depth.init(&mut rng);
    }

    // Check if a given coordinate is part of a monolith
    #[inline(always)]
    pub fn is_monolith(&self, x: i32, z: i32) -> bool {
        if cfg!(feature="only_hill") {
            self.hill.is_hill_monolith(x, z)
        } else {
            self.hill.is_hill_monolith(x, z) &&
            self.depth.is_depth_monolith(x, z)
        }
    }

    /// Get a Monolith at a given coordinate, compute properties
    /// Todo: Arc Mutex HashMap (x, y) => Monolith struct?
    #[inline(always)]
    pub fn get_monolith(&self, x: i32, z: i32) -> Option<Monolith> {

        // Most blocks are not monoliths
        if !self.is_monolith(x, z) {
            return None;
        }

        // How accurate the area calculation is
        let step: i32 = if cfg!(feature="fast-area") {4} else {1};

        let x = utils::nearest(x, step);
        let z = utils::nearest(z, step);
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
        let far: i32 = 256;
        for dx in (-far..=far).step_by(32) {
            for dz in (-far..=far).step_by(32) {
                if (dx*dx + dz*dz) < far*far {
                    queue.push_back((x+dx, z+dz));
                }
            }
        }

        while let Some((x, z)) = queue.pop_front() {
            if !self.is_monolith(x, z) {
                continue;
            }

            lith.area += (step*step) as u64;

            // Check neighbors with step 4 per hill/depth scaling
            let mut neighbors = vec!(
                (0,  step), ( step, 0),
                (0, -step), (-step, 0)
            );

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
                lith.minx = lith.minx.min(x);
                lith.maxx = lith.maxx.max(x);
                lith.minz = lith.minz.min(z);
                lith.maxz = lith.maxz.max(z);
            }

            for (dx, dz) in neighbors {
                let next = (x+dx, z+dz);

                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }

        Some(lith)
    }

    #[inline(always)]
    pub fn find_monoliths(&self, query: &FindOptions) -> Vec<Monolith> {
        let xrange: Vec<i32> = (query.minx..=query.maxx).step_by(query.step).collect();
        let zrange: Vec<i32> = (query.minz..=query.maxz).step_by(query.step).collect();

        // Use non-threaded approach for small areas (lower latency)
        if (query.maxx - query.minx).abs() < 100000 {
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

    /// Lightweight alternative to `find_monoliths()`, shall only return one
    #[inline(always)]
    pub fn find_monolith(&self, query: &FindOptions) -> Option<Monolith> {
        for x in (query.minx..=query.maxx).step_by(query.step) {
            for z in (query.minz..=query.maxz).step_by(query.step) {
                if let Some(mono) = self.get_monolith(x, z) {
                    return Some(mono);
                }
            }
        }
        return None;
    }

    /// Heuristic on the discovered correlation between forcing the fractional
    /// part of the perlin noise to zero yielding no monoliths at spawn, and
    /// semi-surprisingly, generating enormous ones at 0.5 decimals.
    ///
    /// For a normal curve P(x) = exp(-((x-u)/2s)^2), the coefficients for
    /// rolling 'good'  fractional perlin noises seeds are, for (A=10, B=16):
    ///
    /// 1. Unscaled deviations for all octaves:
    ///    - [Hill+Depth](https://www.desmos.com/calculator/w1wwgd3cli)
    ///      - u = ((A+B)/4)      = 19.500
    ///      - s = sqrt((A+B)/48) = 1.2747
    ///    - [Hill Only](https://www.desmos.com/calculator/igbdsm3yat)
    ///      - u = (A/4)      = 7.5000
    ///      - s = sqrt(A/48) = 0.7905
    ///
    /// 2. Scaled deviations for all octaves:
    ///    - [Hill+Depth](https://www.desmos.com/calculator/38jyo6x2lc)
    ///      - u =    3*(2**A - 1 + 2**B - 1)/4   =~ 49918.50
    ///      - s = sqrt((4**A - 1 + 4**B - 1)/48) =~  9460.46
    ///    - [Hill Only](https://www.desmos.com/calculator/odkdrmsf0r)
    ///      - u =   3*(2**A - 1)/4    =~ 767.25
    ///      - s = sqrt((4**A - 1)/48) =~ 147.80
    ///
    #[inline(always)]
    pub fn good_perlin_fracts(seed: u64) -> bool {
        let mut rng = JavaRNG::new(seed);
        Perlin::discard(&mut rng, 48);

        // How good the seed is/should be
        let mut deviate = 0.0;

        // Heuristic numbers to filter out 'bad' seeds
        let (quality, noises): (f64, &[usize]);

        if cfg!(feature="depth-fracts") {
            quality = if cfg!(feature="scaled-deviation") {28000.0} else {16.0};
            noises  = &[HILL_OCTAVES, DEPTH_OCTAVES];
        } else {
            quality = if cfg!(feature="scaled-deviation") {  380.0} else { 5.4};
            noises  = &[HILL_OCTAVES];
        };

        // Simulate parts of perlin initialization
        for octaves in noises {
            for _oct in 0..(*octaves) {

                #[cfg(feature="scaled-deviation")]
                let scale = FractalPerlin::<0>::octave_scale(_oct);

                for _ in 0..3 {
                    let next = rng.next_f64() * 256.0;
                    let next = (0.5 - (next - next.floor())).abs();
                    #[cfg(feature="scaled-deviation")]
                    let next = next * scale;
                    deviate += next;
                }

                // Early exit past treshold
                if quality < deviate {
                    return false;
                }
                rng.step_n(256);
            }
        }

        return true;
    }
}

/* -------------------------------------------------------------------------- */

#[derive(SmartDefault)]
pub struct FindOptions {
    pub minx: i32,
    pub maxx: i32,
    pub minz: i32,
    pub maxz: i32,

    /// Probe the world every N blocks
    #[default(32)]
    pub step: usize,

    /// How many monoliths to find
    pub limit: Option<u64>,
}

impl FindOptions {

    pub fn step(mut self, step: usize) -> Self {
        self.step = step;
        return self;
    }

    pub fn limit(mut self, many: u64) -> Self {
        self.limit = Some(many);
        return self;
    }

    // Defining regions

    /// Search around a given coordinate at most `radius` manhattan blocks away
    pub fn around(mut self, x: i32, z: i32, radius: i32) -> Self {
        self.minx = x - radius;
        self.maxx = x + radius;
        self.minz = z - radius;
        self.maxz = z + radius;
        return self;
    }

    /// Search around spawn at most `radius` manhattan blocks away
    pub fn spawn(self, radius: i32) -> Self {
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
        self.maxx = MONOLITHS_REPEAT;
        self.minz = 0;
        self.maxz = MONOLITHS_REPEAT;
        return self;
    }
}
