fn parse_digit(c: char) -> Result<u8, ()> {
    Ok(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => Err(())?,
    })
}

pub fn solve() -> u64 {
    let digits: Vec<u8> = include_str!("../data/8.txt")
        .chars()
        .filter_map(|c| parse_digit(c).ok())
        .collect();

    digits
        .windows(13)
        .map(|s| s.iter().map(|d| u64::from(*d)).product())
        .max()
        .unwrap()
}
