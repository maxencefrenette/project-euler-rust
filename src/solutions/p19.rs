use chrono::{Datelike, NaiveDate, Weekday};
use itertools::Itertools;

pub fn solve() -> u64 {
    (1901..2000)
        .cartesian_product(1..=12)
        .filter(|(y, m)| NaiveDate::from_ymd(y - 1, *m, 1).weekday() == Weekday::Sun)
        .count() as u64
}
