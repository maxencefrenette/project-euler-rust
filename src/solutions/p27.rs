use itertools::Itertools;
use primal::Sieve;

pub fn solve() -> isize {
    let sieve = Sieve::new(1_000_000);
    let (a, b) = (-1000..=1000)
        .cartesian_product(-1000..=1000)
        .max_by_key(|(a, b)| {
            (0..)
                .take_while(|&n| {
                    let f_n = n * n + a * n + b;
                    f_n > 0 && sieve.is_prime(f_n as usize)
                })
                .count()
        })
        .unwrap();

    a * b
}
