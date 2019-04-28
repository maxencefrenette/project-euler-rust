use float_ord::FloatOrd;
use log::info;

pub fn solve() -> u64 {
  let (n, d) = (2..=1_000_000u64)
    .map(|d| {
      let n = ((3. / 7.) * (d as f64) - 1.).ceil() as u64;
      (n, d)
    })
    .max_by_key(|(n, d)| FloatOrd((*n as f64) / (*d as f64)))
    .unwrap();

  info!("{}/{}", n, d);

  n
}
