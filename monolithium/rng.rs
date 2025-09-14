// This file started as a copy of https://github.com/coderbot16/java-rand, with
// unused parts removed, speed improvements at less safety, new functions to
// discard the next step quickly, that weren't possible to directly modify
// or extend in the original crate, per practical rust limitations.

pub const M: i64 = (1 << 48) - 1;
pub const A: i64 = 0x5DEECE66D;
pub const C: i64 = 11;

pub struct JavaRNG {
    state: i64,
}

impl JavaRNG {

    #[inline(always)]
    pub fn new(seed: u64) -> Self {
        Self {state: ((seed as i64) ^ A) & M}
    }

    /// Roll the state, same effect as ignoring a `.next()` call
    #[inline(always)]
    pub fn step(&mut self) {
        self.state = self.state.wrapping_mul(A).wrapping_add(C) & M
    }

    /// Rolls the state and returns N low bits
    #[inline(always)]
    pub fn next<const BITS: u8>(&mut self) -> i32 {
        self.step();
        return (self.state >> (48 - BITS)) as i32;
    }

    /// Returns a pseudo-random i32 in the range [0, max)
    #[inline(always)]
    pub fn next_i32_bound(&mut self, max: i32) -> i32 {
        if (max as u32).is_power_of_two() {
            (((max as i64).wrapping_mul(self.next::<31>() as i64)) >> 31) as i32
        } else {
            let mut next = self.next::<31>();
            let mut take = next % max;

            while next.wrapping_sub(take).wrapping_add(max - 1) < 0 {
                next = self.next::<31>();
                take = next % max;
            }

            return take;
        }
    }

    /// Accurate, slightly faster than `.next_i32_bound()`. Does not compute
    /// the returned value, just rolls the state as if it did.
    #[inline(always)]
    pub fn skip_next_i32_bound(&mut self, max: i32) {
        if (max as u32).is_power_of_two() {
            self.step();
        } else {
            let mut next = self.next::<31>();
            let mut take = next % max;

            while next.wrapping_sub(take).wrapping_add(max - 1) < 0 {
                next = self.next::<31>();
                take = next % max;
            }
        }
    }

    /// Returns a pseudo-random f64 in the range [0, 1)
    #[inline(always)]
    pub fn next_f64(&mut self) -> f64 {
        let high = (self.next::<26>() as i64) << 27;
        let low  =  self.next::<27>() as i64;
        const MAGIC: f64 = (1u64 << 53) as f64;
        (high | low) as f64 / MAGIC
    }
}

/* -------------------------------------------------------------------------- */

use std::sync::OnceLock;

static SKIP_TABLE_SIZE: usize = 16_384;
static SKIP_TABLE: OnceLock<[(i64, i64); SKIP_TABLE_SIZE]> = OnceLock::new();

impl JavaRNG {

    // Roll the state N times, fast
    #[inline(always)]
    pub fn step_n(&mut self, n: usize) {
        debug_assert!(n < SKIP_TABLE_SIZE);
        if n == 0 {return;}
        let (a_n, c_n) = SKIP_TABLE.get().unwrap()[n];
        self.state = (self.state.wrapping_mul(a_n).wrapping_add(c_n)) & M;
    }

    pub fn init_skip_table() {
        SKIP_TABLE.get_or_init(|| {
            let mut table = [(0i64, 0i64); SKIP_TABLE_SIZE];

            // Start with the identity
            table[0] = (1, 0);

            // Precompute N steps of the LCN
            for n in 1..SKIP_TABLE_SIZE {
                let (p_a, p_c) = table[n - 1];
                let n_a = (p_a.wrapping_mul(A)) & M;
                let n_c = (p_c.wrapping_mul(A).wrapping_add(C)) & M;

                table[n] = (n_a, n_c);
            }

            table
        });
    }

}