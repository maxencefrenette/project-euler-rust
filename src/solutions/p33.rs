use itertools::Itertools;
use num::rational::Rational64;
use num::traits::identities::One;

pub fn solve() -> i64 {
    let digits = vec![1..=9, 0..=9, 1..=9, 0..=9]
        .into_iter()
        .multi_cartesian_product();
    let fractions = digits.filter_map(|abcd| {
        let a = abcd[0];
        let b = abcd[1];
        let c = abcd[2];
        let d = abcd[3];

        // Trivial cases
        if b == 0 && d == 0 {
            return None;
        }

        let frac = Rational64::new(10 * a + b, 10 * c + d);

        if frac >= Rational64::one() {
            return None;
        }

        if frac == Rational64::new(a, c) && b == d
            || frac == Rational64::new(b, c) && a == d
            || d != 0
                && (frac == Rational64::new(a, d) && b == c
                    || frac == Rational64::new(b, d) && a == c)
        {
            dbg!((10 * a + b, 10 * c + d));
            dbg!(frac);
            Some(frac)
        } else {
            None
        }
    });

    *fractions.product::<Rational64>().denom()
}
