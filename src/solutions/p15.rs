use cached::cached;

cached! {
    PATHS;
    fn paths(a: u64, b: u64) -> u64 = {
        if a == 0 || b == 0 {
            return 1;
        }

        paths(a-1, b) + paths(a, b-1)
    }
}

pub fn solve() -> u64 {
    paths(20, 20)
}
