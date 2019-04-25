use gcd::Gcd;

pub fn solve() -> u64 {
    (4u64..=12000)
        .map(|n| {
            let lo = (n as f64 / 3. + 1.).floor() as u64;
            let hi = (n as f64 / 2. - 1.).ceil() as u64;

            (lo..=hi).filter(|d| n.gcd(*d) == 1).count() as u64
        })
        .sum()
}
