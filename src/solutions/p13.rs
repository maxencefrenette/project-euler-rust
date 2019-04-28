pub fn solve() -> u64 {
    let sum: f64 = include_str!("../data/13.txt")
        .split_whitespace()
        .map(|s| s[..13].parse::<f64>().unwrap())
        .sum();

    (sum / 100_000.).round() as u64
}
