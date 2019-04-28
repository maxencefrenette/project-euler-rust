use itertools::Itertools;
use primal::as_perfect_power;

pub fn solve() -> usize {
    let mut powers: Vec<(u64, u64)> = (2u64..=100)
        .cartesian_product(2u64..=100)
        .map(|(a, b)| {
            let (y, k) = as_perfect_power(a);
            (y, b * u64::from(k))
        })
        .collect();
    powers.sort();
    powers.dedup();
    powers.len()
}
