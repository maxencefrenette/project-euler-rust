pub fn solve() -> u64 {
    let sum_of_square: u64 = (1..=100).map(|n| n * n).sum();
    let sum: u64 = (1..=100).sum();
    let square_of_sum = sum * sum;

    square_of_sum - sum_of_square
}
