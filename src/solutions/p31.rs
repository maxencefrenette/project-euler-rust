pub fn solve() -> u64 {
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut combinations = vec![0u64; 201];
    combinations[0] = 1;

    for &c in &coins {
        for i in c..=200 {
                combinations[i] += combinations[i - c];
        }
    }

    combinations[200]
}
