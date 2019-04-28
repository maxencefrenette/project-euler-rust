use primal::Sieve;

pub fn solve() -> u64 {
    let sieve = Sieve::new(2_000_000);

    sieve
        .primes_from(2)
        .map(|n| n as u64)
        .take_while(|&n| n < 2_000_000)
        .sum()
}
