pub fn solve() -> u64 {
    let mut words = include_str!("../data/22.txt")
        .split(',')
        .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
        .collect::<Vec<String>>();

    words.sort();

    words
        .into_iter()
        .enumerate()
        .map(|(n, s)| {
            (n as u64 + 1)
                * s.chars()
                    .map(|c| u64::from(c.to_digit(36).unwrap()) - 9)
                    .sum::<u64>()
        })
        .sum()
}
