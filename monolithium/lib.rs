pub use std::cmp::max;
pub use std::cmp::min;
pub use std::cmp::Ordering;
pub use std::collections::VecDeque;
pub use std::hash::Hash;
pub use std::hash::Hasher;
pub use std::sync::Arc;
pub use std::sync::Mutex;

pub use ahash::AHashSet;
pub use java_rand::Random;
pub use rayon::prelude::*;

pub mod monolith;
pub use monolith::*;
pub mod perlin;
pub use perlin::*;
pub mod utils;
pub mod world;
pub use world::*;

pub const FARLANDS:   i64 = 12_550_824;
pub const WORLD_SIZE: i64 = 2*FARLANDS + 1;
