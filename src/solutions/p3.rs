pub fn solve() -> u64 {
    let mut n = 600_851_475_143;
    let mut sqrt_n = (n as f64).sqrt() as u64;
    let mut i = 3;

    // Iteratively removes the smallest prime factor of n.
    // In the end, we must be left with a prime number that is n's biggest factor
    while i <= sqrt_n {
        if n % i == 0 {
            n /= i;
            sqrt_n = (n as f64).sqrt() as u64;
        }

        i += 2;
    }

    n
}
