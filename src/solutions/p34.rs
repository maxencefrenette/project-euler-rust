use crate::common::digits;
use crate::common::factorial;
use log::info;

pub fn solve() -> u64 {
    (10u64..1_000_000)
        .filter(|&n| n == digits(n).map(factorial).sum())
        .inspect(|&n| {
            info!("{}", n);
        })
        .sum()
}
