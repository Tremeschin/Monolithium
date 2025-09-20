// This file started as a copy of https://github.com/coderbot16/java-rand, with
// unused parts removed, speed improvements at less safety, new functions to
// discard the next step quickly, that weren't possible to directly modify
// or extend in the original crate, per practical rust limitations.

const F: f64 = (1u64 << 53) as f64;
const M: u64 = (1 << 48) - 1;
const A: u64 = 0x5DEECE66D;
const C: u64 = 11;

pub struct JavaRNG {
    state: u64,
}

impl JavaRNG {

    #[inline(always)]
    pub fn new(seed: u64) -> Self {
        Self {state: (seed ^ A) & M}
    }

    /// Roll the state, same effect as ignoring a `.next()` call
    #[inline(always)]
    pub fn step(&mut self) {
        self.state = self.state.wrapping_mul(A).wrapping_add(C) & M
    }

    /// Rolls the state and returns N<=32 low bits
    #[inline(always)]
    pub fn next<const BITS: u8>(&mut self) -> i32 {
        debug_assert!(BITS <= 32);
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

            if cfg!(not(feature="skip-rejection")) {
                while next.wrapping_sub(take).wrapping_add(max - 1) < 0 {
                    next = self.next::<31>();
                    take = next % max;
                }
            }

            return take;
        }
    }

    /// Returns a pseudo-random f64 in the range [0, 1)
    #[inline(always)]
    pub fn next_f64(&mut self) -> f64 {
        let high = (self.next::<26>() as i64) << 27;
        let low  =  self.next::<27>() as i64;
        (high | low) as f64 / F
    }
}

/* -------------------------------------------------------------------------- */

static SKIP_TABLE_SIZE: usize = 16_384;
static SKIP_TABLE: [(u64, u64); SKIP_TABLE_SIZE] = {
    let mut table = [(0u64, 0u64); SKIP_TABLE_SIZE];

    // Start with the identity
    let (mut mul, mut add) = (1, 0);

    // Precompute N steps of the LCG
    let mut n = 0;
    while n < SKIP_TABLE_SIZE {
        table[n] = (mul, add);
        mul = (mul.wrapping_mul(A)) & M;
        add = (add.wrapping_mul(A).wrapping_add(C)) & M;
        n += 1;
    }
    table
};

impl JavaRNG {

    /// Roll the state N times, fast
    #[inline(always)]
    pub fn step_n(&mut self, n: usize) {
        if cfg!(feature="skip-table") {
            debug_assert!(n < SKIP_TABLE_SIZE);
            let (a_n, c_n) = unsafe {SKIP_TABLE.get_unchecked(n)};
            self.state = (self.state.wrapping_mul(*a_n).wrapping_add(*c_n)) & M;
        } else {
            for _ in 0..n {
                self.step();
            }
        }
    }
}
