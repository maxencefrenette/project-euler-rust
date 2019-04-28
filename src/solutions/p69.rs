use crate::common::totient;
use float_ord::FloatOrd;
use primal::Sieve;

pub fn solve() -> u64 {
    let sieve = Sieve::new(1000);

    println!("{}", sieve.upper_bound());

    (2..=1_000_000)
        .max_by_key(|n| FloatOrd((*n as f64) / (totient(*n, &sieve) as f64)))
        .unwrap()
}
