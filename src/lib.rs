#![doc(html_root_url = "https://docs.rs/anyslot/0.1.0")]
//! global bridge c-bindings of any type slot for Rust
//!

pub mod anyslot;

pub mod minimum;

#[cfg(test)]
mod tests {
  use super::minimum::minimum_test;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_anyslot() {
    assert_eq!(minimum_test(), ());
  }
}
