pub fn max_path(raw_data: &str) -> u64 {
    let mut triangle = raw_data
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    while triangle.len() > 1 {
        let last_row = triangle.pop().unwrap();
        let previous = triangle.last_mut().unwrap();

        for i in 0..previous.len() {
            previous[i] += u64::max(last_row[i], last_row[i + 1]);
        }
    }

    triangle[0][0]
}

pub fn solve() -> u64 {
    max_path(include_str!("../data/18.txt"))
}
