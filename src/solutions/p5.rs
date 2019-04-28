use gcd::Gcd;

fn lcm(a: u64, b: u64) -> u64 {
    a * b / a.gcd(b)
}

pub fn solve() -> u64 {
    (1..=20).fold(1, lcm)
}
