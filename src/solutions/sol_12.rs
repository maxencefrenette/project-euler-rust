use primal::Sieve;

pub fn solve() -> u64 {
    let sieve = Sieve::new(10_000);

    for n in (1..).map(|n| n * (n + 1) / 2) {
        let div_count: usize = sieve
            .factor(n as usize)
            .unwrap()
            .into_iter()
            .map(|(_p, exp)| exp + 1)
            .product();

        println!("{:7}: {}", n, div_count);

        if div_count >= 500 {
            return n;
        }
    }

    unreachable!();
}
