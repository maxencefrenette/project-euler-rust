pub fn solve() -> u64 {
    let mut num_solutions = vec![0; 1002];

    for a in 1..1000 {
        let mut b = a;
        loop {
            let c2 = a * a + b * b;
            let c = (c2 as f64).sqrt() as u64;

            let p = a + b + c;
            if p > 1000 {
                break;
            }

            if c * c != c2 {
                b += 1;
                continue;
            }

            num_solutions[p as usize] += 1;

            b += 1;
        }
    }

    (0u64..1002)
        .max_by_key(|&i| num_solutions[i as usize])
        .unwrap()
}
