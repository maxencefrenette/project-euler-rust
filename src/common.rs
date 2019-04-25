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
