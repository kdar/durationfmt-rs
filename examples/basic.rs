extern crate durationfmt;

use std::time::Duration;

fn main() {
  let d = Duration::new(0, 0);
  println!("{}", durationfmt::to_string(d));
  // 0s
  let d = Duration::new(90, 0);
  println!("{}", durationfmt::to_string(d));
  // 1m30s
  let d = Duration::new(209, 1_000);
  println!("{}", durationfmt::to_string(d));
  // 3m29.000001s
}
