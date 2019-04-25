use primal::StreamingSieve;

pub fn solve() -> u64 {
    StreamingSieve::nth_prime(10001) as u64
}
