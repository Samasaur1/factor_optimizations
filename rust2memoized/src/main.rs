use std::{cmp::max, collections::HashMap};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct BitSet(u64);

impl BitSet {
    pub fn contains(&self, value: u64) -> bool {
        value < 64 && (self.0 & (1 << value)) != 0
    }
}

fn factor_exists(num: u64, pool: BitSet) -> bool {
    for i in pool {
        if num != i && num % i == 0 {
            return true;
        }
    }
    false
}

fn remove_factors(num: u64, pool: BitSet) -> BitSet {
    let mut res = 0;
    for i in 1..63 {
        if num % i != 0 {
            res |= 1 << i;
        }
    }
    BitSet(pool.0 & res)
}

fn solve_memo(memo: &mut HashMap<BitSet, u64>, pool: BitSet) -> u64 {
    if let Option::Some(&val) = memo.get(&pool) {
        val
    } else {
        let mut best = 0;
        for i in pool {
            if factor_exists(i, pool) {
                let new_pool = remove_factors(i, pool);
                let result = solve_memo(memo, new_pool);
                best = max(best, result + i);
            }
        }
        memo.insert(pool, best);
        best
    }
}

fn solve(pool: BitSet) -> u64 {
    solve_memo(&mut HashMap::new(), pool)
}

fn main() {
    let n = 24;
    println!("{}", solve(BitSet(((1 << (n + 1)) - 1) & (!1))));
}

impl IntoIterator for BitSet {
    type Item = u64;

    type IntoIter = BSIter;

    fn into_iter(self) -> Self::IntoIter {
        BSIter { bs: self, next: 0 }
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
