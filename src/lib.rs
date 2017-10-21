// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::time::Duration;

// Returns a string representing the duration in the form "72h3m0.5s".
// Leading zero units are omitted. As a special case, durations less than one
// second format use a smaller unit (milli-, micro-, or nanoseconds) to ensure
// that the leading digit is non-zero. The zero duration formats as 0s.
pub fn to_string(d: Duration) -> String {
  let mut buf = vec![0u8; 32];
  let mut w = buf.len();

  let secs = d.as_secs();
  let nanos = d.subsec_nanos();

  if secs == 0 {
    // Special case: if duration is smaller than a second,
    // use smaller units, like 1.2ms

    let mut prec = 0;

    w -= 1;
    buf[w] = b's';
    w -= 1;

    if nanos == 0 {
      return "0s".to_owned();
    } else if nanos < 1_000 {
      // print nanoseconds
      prec = 0;
      buf[w] = b'n';
    } else if nanos < 1_000_000 {
      // print microseconds
      prec = 3;
      buf[w] = 0xb5;
      w -= 1;
      buf[w] = 0xc2;
    } else {
      // print milliseconds
      prec = 6;
      buf[w] = b'm';
    }

    let (mut w, u) = fmt_frac(&mut buf[..w], nanos as u64, prec);
    w = fmt_int(&mut buf[..w], u);
    return String::from_utf8_lossy(&buf[w..]).into_owned();
  }

  let mut u = secs;
  w -= 1;
  buf[w] = b's';

  let (mut w, _) = fmt_frac(&mut buf[..w], nanos as u64, 9);

  // u is now integer seconds
  w = fmt_int(&mut buf[..w], u % 60);
  u /= 60;

  // u is now integer minutes
  if u > 0 {
    w -= 1;
    buf[w] = b'm';
    w = fmt_int(&mut buf[..w], u % 60);
    u /= 60;

    // u is now integer hours
    // Stop at hours because days can be different lengths.
    if u > 0 {
      w -= 1;
      buf[w] = b'h';
      w = fmt_int(&mut buf[..w], u);
    }
  }

  return String::from_utf8_lossy(&buf[w..]).into_owned();
}

// Formats the fraction of v/10**prec (e.g., ".12345") into the
// tail of buf, omitting trailing zeros. it omits the decimal
// point too when the fraction is 0. It returns the index where the
// output bytes begin and the value v/10**prec.
fn fmt_frac(buf: &mut [u8], v: u64, prec: u8) -> (usize, u64) {
  let mut w = buf.len();
  let mut v = v;

  let mut print = false;
  for _ in 0..prec {
    let digit = v % 10;
    print = print || digit != 0;
    if print {
      w -= 1;
      buf[w] = digit as u8 + b'0';
    }
    v /= 10;
  }

  if print {
    w -= 1;
    buf[w] = b'.';
  }

  (w, v)
}

// Formats v into the tail of buf.
// It returns the index where the output begins.
fn fmt_int(buf: &mut [u8], v: u64) -> usize {
  let mut w = buf.len();
  let mut v = v;

  if v == 0 {
    w -= 1;
    buf[w] = b'0';
  } else {
    while v > 0 {
      w -= 1;
      buf[w] = ((v % 10) as u8) + b'0';
      v /= 10;
    }
  }

  w
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_string() {
    assert_eq!("0s", to_string(Duration::new(0, 0)));
    assert_eq!("1ns", to_string(Duration::new(0, 1)));
    assert_eq!("1.1µs", to_string(Duration::new(0, 1100)));
    assert_eq!("2.2ms", to_string(Duration::new(0, 2200_000)));
    assert_eq!("100.567123ms", to_string(Duration::new(0, 100_567_123)));
    assert_eq!("3.3s", to_string(Duration::new(3, 300_000_000)));
    assert_eq!(
      "9m13.123456789s",
      to_string(Duration::new(553, 123_456_789))
    );
    assert_eq!("4m5s", to_string(Duration::new(4 * 60 + 5, 0)));
    assert_eq!("4m5.001s", to_string(Duration::new(4 * 60 + 5, 1_000_000)));
    assert_eq!(
      "5h6m7.001s",
      to_string(Duration::new((5 * 60 * 60) + (6 * 60) + 7, 1_000_000))
    );
    assert_eq!("8m0.000000001s", to_string(Duration::new(8 * 60, 1)));
    assert_eq!(
      "2562047h47m16.854775807s",
      to_string(Duration::new(
        (2562047 * 60 * 60) + (47 * 60) + 16,
        854_775_807
      ))
    );
    assert_eq!(
      "5124095576030431h0m15s",
      to_string(Duration::new(std::u64::MAX, 0))
    );
    assert_eq!(
      "5124095576030431h0m15.999999999s",
      to_string(Duration::new(std::u64::MAX, 999_999_999))
    );
    assert_eq!(
      "5124095576030431h0m15.000001s",
      to_string(Duration::new(std::u64::MAX, 1_000))
    );
    assert_eq!(
      "5124095576030431h0m15.001s",
      to_string(Duration::new(std::u64::MAX, 1_000_000))
    );
  }
}
