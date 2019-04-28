use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::identities::Zero;
use num_traits::pow::Pow;

pub fn solve() -> u64 {
    let mut n: BigUint = BigUint::from(2u64).pow(1000u64);
    let mut sum = 0;

    while !n.is_zero() {
        let r = &n % BigUint::from(10u64);
        sum += r.to_u64().unwrap();
        n -= r;
        n /= 10u64;
    }

    sum
}
