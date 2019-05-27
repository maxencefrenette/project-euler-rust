use num::bigint::BigUint;
use num::traits::cast::ToPrimitive;
use num::traits::identities::Zero;
use primal::Sieve;
use std::ops::Mul;

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

pub struct Digits {
    num: u64,
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.num != 0 {
            let rem = self.num % 10;
            self.num /= 10;
            Some(rem)
        } else {
            None
        }
    }
}

pub fn digits(num: u64) -> Digits {
    Digits { num }
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

pub fn sum_digits(mut n: BigUint) -> u64 {
    let mut sum = 0;

    while !n.is_zero() {
        let r = &n % BigUint::from(10u64);
        sum += r.to_u64().unwrap();
        n -= r;
        n /= 10u64;
    }

    sum
}

pub fn factorial(n: u64) -> u64 {
    (1..=n).fold(1, u64::mul)
}

/// Computes n^k mod m
pub fn mod_pow(n: u64, k: u64, m: u64) -> u64 {
    if k == 0 {
        return 1;
    } else if k == 1 {
        return n;
    }

    if k % 2 == 0 {
        mod_pow((n * n) % m, k / 2, m)
    } else {
        (k * mod_pow(n, k - 1, m)) % m
    }
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
