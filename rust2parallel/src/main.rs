use dashmap::DashMap;
use rayon::prelude::*;
use std::ops::{BitAnd, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct BitSet(u64);

impl BitSet {
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub const fn singleton(n: u64) -> Self {
        Self(1 << n)
    }

    pub const fn contains(&self, value: u64) -> bool {
        value < 64 && (self.0 & (1 << value)) != 0
    }
}

const FACTORS: [BitSet; 64] = factor_list();

const fn factors(val: u64) -> BitSet {
    let mut f = 0;
    let mut i = 1;
    while i < val {
        if val % i == 0 {
            f |= 1 << i;
        }
        i += 1;
    }
    BitSet(f)
}

const fn factor_list<const N: usize>() -> [BitSet; N] {
    let mut vals = [BitSet(0); N];
    let mut i = 1;
    while i < N {
        vals[i] = factors(i as u64);
        i += 1;
    }
    vals
}

fn factor_exists(num: u64, pool: BitSet) -> bool {
    !(pool & FACTORS[num as usize]).is_empty()
}

fn remove_factors(num: u64, pool: BitSet) -> BitSet {
    pool - FACTORS[num as usize] - BitSet::singleton(num)
}

fn solve_memo(memo: &DashMap<BitSet, u64>, pool: BitSet) -> u64 {
    if let Option::Some(val) = memo.get(&pool) {
        *val
    } else {
        let best = pool.into_iter()
            .collect::<Vec<_>>()
            .into_par_iter()
            .filter(|&i| factor_exists(i, pool))
            .map(|i| solve_memo(memo, remove_factors(i, pool)) + i)
            .max()
            .unwrap_or(0);
        
        memo.insert(pool, best);
        best
    }
}

fn solve(pool: BitSet) -> u64 {
    solve_memo(&DashMap::new(), pool)
}

fn main() {
    let n = 25;
    println!("{}", solve(BitSet(((1 << (n + 1)) - 1) & (!1))));
}

impl IntoIterator for BitSet {
    type Item = u64;

    type IntoIter = BSIter;

    fn into_iter(self) -> Self::IntoIter {
        BSIter { bs: self, next: 0 }
    }
}

impl BitAnd<BitSet> for BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: BitSet) -> Self::Output {
        BitSet(self.0 & rhs.0)
    }
}

impl Sub<BitSet> for BitSet {
    type Output = BitSet;

    fn sub(self, rhs: BitSet) -> Self::Output {
        BitSet(self.0 & !rhs.0)
    }
}

struct BSIter {
    bs: BitSet,
    next: u64,
}

impl Iterator for BSIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < 64 {
            let next = self.next;
            self.next += 1;
            if self.bs.contains(next) {
                return Some(next);
            }
        }
        None
    }
}
