use float_ord::FloatOrd;
use primal::Sieve;

fn phi(n: u64, sieve: &Sieve) -> u64 {
    let factors = sieve.factor(n as usize).unwrap();

    factors
        .into_iter()
        .map(|(p_usize, k_usize)| {
            let p = p_usize as u64;
            let k = k_usize as u32;

            p.pow(k) - p.pow(k - 1)
        })
        .product()
}

pub fn solve() -> u64 {
    let sieve = Sieve::new(1000);

    println!("{}", sieve.upper_bound());

    (2..=1_000_000)
        .max_by_key(|n| FloatOrd((*n as f64) / (phi(*n, &sieve) as f64)))
        .unwrap()
}
