use crate::common::sum_digits;

pub fn solve() -> u64 {
    sum_digits((1u32..100).product())
}
