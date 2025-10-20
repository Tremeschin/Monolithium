// This file started as a copy of https://github.com/coderbot16/java-rand, with
// unused parts removed, speed improvements at less safety, new functions to
// discard the next step quickly, that weren't possible to directly modify
// or extend in the original crate, per practical rust limitations.

const F: f64 = (1u64 << 53) as f64;
const M: u64 = (1 << 48) - 1;
const A: u64 = 0x5DEECE66D;
const C: u64 = 11;

// Modular multiplicative inverse constants
const AI: u64 = 0xDFE05BCB1365;
const CI: u64 = (M + 1).wrapping_sub(C).wrapping_mul(AI) & M;

/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct JavaRNG {
    state: u64,
}

impl JavaRNG {

    #[inline(always)]
    pub fn from_seed(seed: u64) -> Self {
        Self {state: (seed ^ A) & M}
    }

    #[inline(always)]
    pub fn from_state(state: u64) -> Self {
        Self {state: state & M}
    }

    /// Find a seed that starts off at the current state
    #[inline(always)]
    pub fn reverse_seed(&self) -> u64 {
        self.state ^ A
    }

    /// Roll the state, same effect as ignoring a `.next()` call
    #[inline(always)]
    pub fn step(&mut self) {
        self.state = self.state.wrapping_mul(A).wrapping_add(C) & M
    }

    /// Roll the state backwards, undoing a `.next()` call
    #[inline(always)]
    pub fn back(&mut self) {
        self.state = AI.wrapping_mul(self.state.wrapping_sub(C)) & M;
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

static SKIP_TABLE_SIZE: usize = 2_usize.pow(15);

/// Forward modular multiplication table
static SKIP_TABLE_NEXT: [(u64, u64); SKIP_TABLE_SIZE] = {
    let mut table = [(0u64, 0u64); SKIP_TABLE_SIZE];
    let (mut mul, mut add) = (1, 0);
    let mut n = 0;
    while n < SKIP_TABLE_SIZE {
        table[n] = (mul, add);
        mul = (mul.wrapping_mul(A)) & M;
        add = (add.wrapping_mul(A).wrapping_add(C)) & M;
        n += 1;
    }
    table
};

/// Modular multiplicative inverse table
static SKIP_TABLE_BACK: [(u64, u64); SKIP_TABLE_SIZE] = {
    let mut table = [(0u64, 0u64); SKIP_TABLE_SIZE];
    let (mut mul, mut add) = (1, 0);
    let mut n = 0;
    while n < SKIP_TABLE_SIZE {
        table[n] = (mul, add);
        mul = (mul.wrapping_mul(AI)) & M;
        add = (add.wrapping_mul(AI).wrapping_add(CI)) & M;
        n += 1;
    }
    table
};

impl JavaRNG {

    /// Roll the state N times fast (lossy)
    #[inline(always)]
    pub fn step_n(&mut self, n: usize) {
        if cfg!(feature="skip-table") {
            debug_assert!(n < SKIP_TABLE_SIZE);
            let (a_n, c_n) = unsafe {SKIP_TABLE_NEXT.get_unchecked(n)};
            self.state = (self.state.wrapping_mul(*a_n).wrapping_add(*c_n)) & M;
        } else {
            for _ in 0..n {
                self.step();
            }
        }
    }

    /// Roll the state backwards N times fast (lossy)
    #[inline(always)]
    pub fn back_n(&mut self, n: usize) {
        if cfg!(feature="skip-table") {
            debug_assert!(n < SKIP_TABLE_SIZE);
            let (a_n, c_n) = unsafe {SKIP_TABLE_BACK.get_unchecked(n)};
            self.state = (self.state.wrapping_mul(*a_n).wrapping_add(*c_n)) & M;
        } else {
            for _ in 0..n {
                self.back();
            }
        }
    }
}
