use crate::common::sum_digits;
use num_bigint::BigUint;
use num_traits::pow::Pow;

pub fn solve() -> u64 {
    sum_digits(BigUint::from(2u64).pow(1000u64))
}
