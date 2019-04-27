use primal::{Primes, Sieve};

pub fn solve() -> u64 {
    let b = 10u64.pow(16);

    let primes = Primes::all()
        .take_while(|p| *p < 5_000)
        .collect::<Vec<usize>>();

    let sum = primes.iter().sum();

    let sieve = Sieve::new(sum);

    let mut combinations: Vec<u64> = vec![1];
    combinations.resize(sum + 1, 0);

    for p in primes.iter() {
        for i in (*p..combinations.len()).rev() {
            combinations[i] = (combinations[i] + combinations[i - p]) % b;
        }
    }

    sieve
        .primes_from(0)
        .take_while(|p| *p < combinations.len())
        .map(|p| combinations[p])
        .sum::<u64>()
        % b
}
