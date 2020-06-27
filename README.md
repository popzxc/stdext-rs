# `std` extensions

**Status:**
[![CI](https://github.com/popzxc/stdext-rs/workflows/CI/badge.svg)](https://github.com/popzxc/stdext-rs/actions)

**Project info:**
[![Docs.rs](https://docs.rs/stdext/badge.svg)](https://docs.rs/stdext)
[![Latest Version](http://meritbadge.herokuapp.com/stdext)](https://crates.io/crates/stdext)
[![License](https://img.shields.io/github/license/popzxc/stdext-rs.svg)](https://github.com/popzxc/stdext-rs)
![Rust 1.44+ required](https://img.shields.io/badge/rust-1.44+-blue.svg?label=Rust)

Additional features for the Rust standard library.

## Description

This crate contains enhancements to the Rust standard library structures, useful for
broad audience, but not yet implemented (or stabilized) in `std`.

Crate is designed to be lightweight (no external dependencies!) and provide essential
functionality which possible can get to the `std` some day.

## Highlights

- Convenient builder methods for **`Duration`**:
  
  ```rust
  use stdext::prelude::*;

  let duration = Duration::from_minutes(1).add_seconds(5).add_micros(100);
  assert_eq!(duration, Duration::new(61, 100_000));
  ```

- Panicking version for **`RwLock::read`**, **`RwLock::write`** and **`Mutex::lock`** (for
  fellows who don't really handle the lock poisoning):

  ```rust
  use std::sync::{Arc, RwLock};
  use stdext::prelude::*;
  
  let lock = Arc::new(RwLock::new(1));
  {
      let mut n = lock.force_write(); // Instead of `.write().unwrap()`.
      *n = 2;
  }
  let n = lock.force_read();
  assert_eq!(*n, 2);
  ```
  
- **`Result::combine`** and **`Option::combine`** to zip pairs of objects:
  
  ```rust
  use stdext::prelude::*;
  
  let x = Some(1);
  let y = Some("hi");
  let z = None::<u8>;
  
  assert_eq!(x.combine(y), Some((1, "hi")));
  assert_eq!(x.combine(z), None);

  let x = Ok(1);
  let y = Ok("hi");
  let z: Result<i32, &str> = Err("error");
  let z2: Result<i32, &str> = Err("other_error");

  assert_eq!(x.combine(y), Ok((1, "hi")));
  assert_eq!(x.combine(z), Err("error"));
  assert_eq!(z.combine(z2), Err("error"));
  ```

...and other features. For a full list, check out the [crate documentation](https://docs.rs/stdext/).

## Motivation

Standard library is great, and it becomes even better through time. However, a time gap between proposing
a new feature and getting it stabilized is way too big.

This crate, however, can be updated quickly and the feature will be usable on the stable Rust soon after
implementation.

## Contributing

If you want to contribute to this project, please read the [contributing guide](CONTRIBUTING.md).

## LICENSE

`stdext` library is licensed under the MIT License. See [LICENSE](LICENSE) for details.
