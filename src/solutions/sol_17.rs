fn spell(n: u64) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        15 => String::from("fifteen"),
        18 => String::from("eighteen"),
        n @ 14...19 => format!("{}teen", spell(n - 10)),
        n @ 20...99 => {
            let words = vec![
                "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
            ];
            let word = String::from(words[((n / 10) - 2) as usize]);

            if n % 10 == 0 {
                word
            } else {
                format!("{}-{}", word, spell(n % 10))
            }
        }
        n @ 100...999 => {
            if n % 100 == 0 {
                format!("{} hundred", spell(n / 100))
            } else {
                format!("{} hundred and {}", spell(n / 100), spell(n % 100))
            }
        }
        1000 => String::from("one thousand"),
        _ => unreachable!(),
    }
}

pub fn solve() -> u64 {
    (1..=1000u64)
        .map(|n| spell(n).chars().filter(|c| c.is_alphabetic()).count() as u64)
        .sum()
}
