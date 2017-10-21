durationfmt
===========

A Rust library to format std::time::Duration the same way Go does.

This is almost a verbatim copy of the algorithm Go uses.

## Usage

Add to your Cargo.toml file:

```toml
[dependencies]
durationfmt = { git = "https://github.com/kdar/durationfmt-rs", branch = "master"}
```

and this to your crate root:

```rust
extern crate durationfmt;
```

## Example

```rust
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
```
