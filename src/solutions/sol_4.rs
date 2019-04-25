fn is_palindrome(mut n: u64) -> bool {
    let mut digits = Vec::new();

    while n != 0 {
        let d = n % 10;
        n -= d;
        n /= 10;
        digits.push(d);
    }

    digits.iter().eq(digits.iter().rev())
}

pub fn solve() -> u64 {
    (100..=999)
        .flat_map(|a| (100..=999).map(move |b| a * b))
        .filter(|n| is_palindrome(*n))
        .max()
        .unwrap()
}
