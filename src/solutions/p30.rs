use crate::common::digits;

pub fn solve() -> u64 {
    let max: u64 = 5 * 9u64.pow(5);

    (2..=max)
        .filter(|n| *n == digits(*n).map(|d| d.pow(5)).sum())
        .sum()
}
