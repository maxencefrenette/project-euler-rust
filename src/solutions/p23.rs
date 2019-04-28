use itertools::Itertools;

pub fn solve() -> u64 {
    let abundant_numbers = (2..=28123)
        .filter(|&n| (1..n).filter(|k| n % k == 0).sum::<u64>() > n)
        .collect::<Vec<u64>>();

    let mut achievable_numbers: Vec<bool> = Vec::new();
    achievable_numbers.resize(28123 + 1, false);

    for (a, b) in abundant_numbers
        .iter()
        .cartesian_product(abundant_numbers.iter())
    {
        let sum = (a + b) as usize;
        if sum < achievable_numbers.len() {
            achievable_numbers[sum] = true;
        }
    }

    achievable_numbers
        .into_iter()
        .enumerate()
        .filter(|(_n, perfect)| !*perfect)
        .map(|(n, _perfect)| n as u64)
        .sum()
}
