use mod_exp::mod_exp;

pub fn solve() -> u64 {
    let modulus = 10_000_000_000;
    let sum: u128 = (1..=1000).map(|n| mod_exp::<u128>(n, n, modulus)).sum();

    (sum % modulus) as u64
}
