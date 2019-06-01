bitset-fixed
===

**Under developing**

Bitset for DP.

## Example

Example for AGC20-C

```rust
use bitset_fixed::BitSet;
use rand::prelude::*;

fn main() {
    let mut rng = StdRng::seed_from_u64(114514);

    let n: Vec<usize> = (0..25).map(|_| rng.next_u32() as usize % 2000).collect();
    let sum = n.iter().sum::<usize>();

    let mut bitset = BitSet::new(sum + 1);
    bitset.set(0, true);

    for &x in &n {
        bitset |= &(bitset.clone() << x);
    }

    let ans = ((sum + 1) / 2..).find(|&i| bitset[i]).unwrap();

    println!("N = {:?}\nAnswer = {}", n, ans);
}
```
