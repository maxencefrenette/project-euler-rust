pub fn solve() -> u32 {
    (0..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}
