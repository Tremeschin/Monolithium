// Note: This file is a copy of https://github.com/coderbot16/java-rand with
// unused parts removed, speed improvements at less safety, new functions to
// discard the next step quickly, that weren't possible to directly modify
// or extend in the original crate, per practical rust limitations.

use std::num::Wrapping;

pub const M: Wrapping<i64> = Wrapping((1 << 48) - 1);
pub const A: Wrapping<i64> = Wrapping(0x5DEECE66D);
pub const C: Wrapping<i64> = Wrapping(11);

const F64_DIV: f64 = (1u64 << 53) as f64;

pub struct JavaRNG {
	state: Wrapping<i64>,
}

impl JavaRNG {
	pub fn new(seed: u64) -> Self {
		JavaRNG {
			state: Wrapping((seed as i64) ^ A.0) & M,
		}
	}

	/// Roll the state, same effect as ignoring a `next`` call
	pub fn step(&mut self) {
		self.state = (self.state * A + C) & M;
	}

	pub fn next(&mut self, bits: u8) -> i32 {
		self.step();
		((self.state.0 as u64) >> (48 - bits)) as i32
	}

	pub fn next_i32_bound(&mut self, max: i32) -> i32 {
		if (max as u32).is_power_of_two() {
			(((max as i64).wrapping_mul(self.next(31) as i64)) >> 31) as i32
		} else {
			let mut next = self.next(31);
			let mut take = next % max;

			while next.wrapping_sub(take).wrapping_add(max - 1) < 0 {
				next = self.next(31);
				take = next % max;
			}

			return take;
		}
	}

	pub fn next_f64(&mut self) -> f64 {
		let high = (self.next(26) as i64) << 27;
		let low  =  self.next(27) as i64;
		(high.wrapping_add(low) as f64) / F64_DIV
	}
}