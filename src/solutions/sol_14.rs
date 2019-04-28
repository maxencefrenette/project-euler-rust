use cached::cached;

cached! {
    COLLATZ_COUNT;
    fn collatz_count(n: u64) -> u64 = {
        if n == 1 {
            return 0;
        }

        if n % 2 == 0 {
            1 + collatz_count(n / 2)
        } else {
            2 + collatz_count((3 * n + 1) / 2)
        }
    }
}

pub fn solve() -> u64 {
    (1..1_000_000).max_by_key(|n| collatz_count(*n)).unwrap()
}
