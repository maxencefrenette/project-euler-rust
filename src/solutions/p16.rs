use crate::common::sum_digits;
use num::bigint::BigUint;
use num::traits::pow::Pow;

pub fn solve() -> u64 {
    sum_digits(BigUint::from(2u64).pow(1000u64))
}
