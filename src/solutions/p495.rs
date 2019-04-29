use crate::common::mod_pow;
use log::info;
use primal::Sieve;

const N: u64 = 10_000;
const K: u64 = 30;
const M: u64 = 1_000_000_007;

pub fn mul(a: u64, b: u64) -> u64 {
    a * b % M
}

pub fn choose(n: u64, k: u64) -> u64 {
    (n..(n - k)).fold(1, mul)
}

pub fn fact(n: u64) -> u64 {
    (1..n).fold(1, mul)
}

pub fn mod_inverse(n: u64) -> u64 {
    // We know M is prime
    mod_pow(n, M - 2, M)
}

/// Still a WIP
pub fn solve() -> u64 {
    let sieve = Sieve::new(N as usize);

    let mut factors: Vec<u64> = Vec::new();
    factors.resize(sieve.prime_pi(N as usize), 0);

    for i in 2..=N {
        for (p, k) in sieve.factor(i as usize).unwrap() {
            factors[sieve.prime_pi(p - 1)] += k as u64;
        }
    }

    info!("{:?}", factors);

    let total_arrangements: u64 = factors
        .into_iter()
        .map(|n| choose(n + K - 1, K - 1))
        .product();

    let fact_30 = fact(30);
    info!("30! = {}", fact_30);

    (total_arrangements * mod_inverse(fact_30)) % M
}
