use primal::Sieve;

pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let tmp = self.a + self.b;
        self.a = self.b;
        self.b = tmp;

        Some(self.b)
    }
}

pub fn fibonacci_iter() -> Fibonacci {
    Fibonacci { a: 0, b: 1 }
}

pub fn totient(n: u64, sieve: &Sieve) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totient_test() {
        let sieve = Sieve::new(30);
        assert_eq!(totient(780, &sieve), 192);
    }
}
