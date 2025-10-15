pub use std::cmp::max;
pub use std::cmp::min;
pub use std::cmp::Ordering;
pub use std::collections::HashSet;
pub use std::collections::VecDeque;
pub use std::hash::Hash;
pub use std::hash::Hasher;
pub use std::sync::Arc;
pub use std::sync::mpsc;
pub use std::sync::Mutex;
pub use std::sync::OnceLock;
pub use std::thread;

pub use ahash::AHashSet;
pub use clap::Parser;
pub use clap::Subcommand;
pub use indicatif::ParallelProgressIterator;
pub use indicatif::ProgressBar;
pub use indicatif::ProgressStyle;
pub use rayon::prelude::*;
pub use serde::Deserialize;
pub use serde::Serialize;
pub use smart_default::SmartDefault;

pub mod commands;
pub mod monolith;
pub mod perlin;
pub mod rng;
pub mod seeds;
pub mod utils;
pub mod world;
pub use monolith::*;
pub use perlin::*;
pub use rng::JavaRNG;
pub use seeds::*;
pub use utils::*;
pub use world::*;

pub type Seed = u64;

/// Coordinate at which the Far Lands start
pub const FARLANDS: i32 = 12_550_824;

/// Lateral size of the inbounds worlds within the Far Lands
pub const WORLD_SIZE: i32 = 2*FARLANDS + 1;

/// Distance in which the hill noise wraps around
pub const HILL_WRAPS: i32 = 2_i32.pow(19);

/// Distance in which the depth noise wraps around
pub const DEPTH_WRAPS: i32 = 2_i32.pow(23);

/// It was found experimentally that the perlin noise and
/// monoliths wraps around every 2**23 blocks, drastically
/// reducing the practical search space!
pub const MONOLITHS_REPEAT: i32 = DEPTH_WRAPS;

/// Java uses a 48-bit Linear Congruential Generator for its RNG,
/// which continuously masks the state's (1 << 48) - 1 lower bits,
/// meaning there's effectively only 2**48 unique seeds!
pub const TOTAL_SEEDS: u64 = 2_u64.pow(48);
