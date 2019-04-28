use std::collections::HashMap;

pub fn solve() -> u64 {
    (2..999)
        .max_by_key(|&n| {
            let mut visited: HashMap<u64, u64> = HashMap::new();
            let mut i = 0;
            let mut remainder = 10;

            while remainder != 0 {
                visited.insert(remainder, i);

                if remainder < n {
                    remainder *= 10;
                } else {
                    remainder %= n;
                    remainder *= 10;
                }

                i += 1;

                if let Some(i_last) = visited.get(&remainder) {
                    return i - i_last;
                }
            }

            0
        })
        .unwrap()
}
