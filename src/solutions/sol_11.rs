pub fn solve() -> u64 {
    let numbers: Vec<u64> = include_str!("../data/11.txt")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let rows = (0..400)
        .filter(|i| i % 20 < 17)
        .map(|i| numbers[i] * numbers[i + 1] * numbers[i + 2] * numbers[i + 3]);
    let columns =
        (0..340).map(|i| numbers[i] * numbers[i + 20] * numbers[i + 40] * numbers[i + 60]);
    let diag1 = (0..337)
        .filter(|i| i % 20 < 17)
        .map(|i| numbers[i] * numbers[i + 21] * numbers[i + 42] * numbers[i + 63]);
    let diag2 = (0..343)
        .filter(|i| i % 20 < 17)
        .map(|i| numbers[i] * numbers[i + 19] * numbers[i + 38] * numbers[i + 57]);

    rows.chain(columns).chain(diag1).chain(diag2).max().unwrap()
}
