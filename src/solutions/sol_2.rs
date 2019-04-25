use crate::common::fibonacci_iter;

pub fn solve() -> u32 {
    fibonacci_iter()
        .take_while(|n| *n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum()
}
