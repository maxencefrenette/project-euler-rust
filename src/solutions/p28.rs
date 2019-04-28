use log::info;

pub fn solve() -> u64 {
    let mut sum = 1;
    let mut cur = 1;

    for i in 1..=500 {
        for _ in 0..4 {
            cur += 2 * i;
            info!("{}", cur);
            sum += cur;
        }
    }

    sum
}
