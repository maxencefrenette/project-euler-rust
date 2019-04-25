pub fn solve() -> u64 {
    for a in 1..=333 {
        for b in a..=500 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }

    0
}
