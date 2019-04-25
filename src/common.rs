pub struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let tmp = self.a + self.b;
        self.a = self.b;
        self.b = tmp;

        Some(self.b)
    }
}

pub fn fibonacci_iter() -> Fibonacci {
    Fibonacci { a: 0, b: 1 }
}
