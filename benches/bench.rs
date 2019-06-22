#![feature(test)]

extern crate test;

use bitset_fixed::BitSet;

#[bench]
fn bench_bitset_dp(b: &mut test::Bencher) {
    use rand::prelude::*;
    let size = 1000;
    let mut v = Vec::new();
    let mut rng = StdRng::seed_from_u64(114514);

    for _ in 0..size {
        v.push(rng.next_u32() as usize % size);
    }

    b.iter(|| {
        let mut bset = BitSet::new(100000);
        bset.set(0, true);

        for &x in &v {
            bset |= &(&bset << x);
        }
    });
}

#[bench]
fn bench_bitset_dp_shl_or(b: &mut test::Bencher) {
    use rand::prelude::*;
    let size = 1000;
    let mut v = Vec::new();
    let mut rng = StdRng::seed_from_u64(114514);

    for _ in 0..size {
        v.push(rng.next_u32() as usize % size);
    }

    b.iter(|| {
        let mut bset = BitSet::new(100000);
        bset.set(0, true);

        for &x in &v {
            bset.shl_or(x);
        }
    });
}
