// The current approach should work, but is equivalent to computing fibonacci numbers.
// If seems that f(n) = fib(n), but it's only true for small numbers
// Theoretically it seems that f(n) should simply be fib(n), but apparently not

#![allow(dead_code)]

use crate::common::fibonacci_iter;
use cached::cached;
use gcd::Gcd;

/// A number of the form a*n + b
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModNumber {
    a: u64,
    b: u64,
}

impl ModNumber {
    fn new(a: u64, b: u64) -> ModNumber {
        ModNumber { a, b }
    }

    fn add(mut self, n: u64) -> ModNumber {
        self.b = (self.b + n) % self.a;
        self
    }

    fn mul(mut self, n: u64) -> ModNumber {
        self.a *= n;
        self.b *= n;
        self
    }

    fn div(mut self, n: u64) -> ModNumber {
        debug_assert!(self.a % n == 0);
        debug_assert!(self.b % n == 0);

        self.a /= n;
        self.b /= n;
        self
    }

    fn is_even(self) -> bool {
        self.a % 2 == 0 && self.b % 2 == 0
    }

    fn is_odd(self) -> bool {
        self.a % 2 == 0 && self.b % 2 == 1
    }

    fn with_n_even(mut self) -> ModNumber {
        self.a *= 2;
        self
    }

    fn with_n_odd(self) -> ModNumber {
        ModNumber::new(2, 1).mul(self.a).add(self.b)
    }

    fn collatz(self) -> ModNumber {
        if self.is_even() {
            self.div(2)
        } else if self.is_odd() {
            self.mul(3).add(1)
        } else {
            unreachable!()
        }
    }
}

cached! {
    F;

    fn f(start: ModNumber, n: u64) -> u64 = {
        let g = start.a.gcd(start.b);
        if !(g == 1 || g == 2) {
            println!("{}n + {}", start.a, start.b);
        }

        if n == 1 {
            if !start.is_even() {
                let last = start.mul(3).add(1);
                if !last.is_odd() {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }

        if start.is_even() || start.is_odd() {
            f(start.collatz(), n - 1)
        } else {
            f(start.with_n_even().collatz(), n - 1) + f(start.with_n_odd().collatz(), n - 1)
        }
    }
}

pub fn solve() -> u64 {
    // Still too slow
    // f(ModNumber::new(1, 0), 90)

    fibonacci_iter().nth(88).unwrap()
}
