fn sum_divisors(n: u64) -> u64 {
    (1..n).filter(|d| n % d == 0).sum()
}

pub fn solve() -> u64 {
    (1..10_000)
        .filter_map(|n| {
            let sum = sum_divisors(n);
            if sum > n {
                let sum2 = sum_divisors(sum);
                if sum2 == n {
                    return Some(n + sum);
                }
            }

            None
        })
        .sum()
}
