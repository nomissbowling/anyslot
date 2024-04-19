//! minimum test for anyslot
//!

use crate::anyslot::*;

/// CustomSlotSample
#[derive(Debug, Clone)]
pub struct CustomSlotSample {
  /// r
  pub r: u8,
  /// g
  pub g: u8,
  /// b
  pub b: u8,
  /// a
  pub a: u8
}

/// TBridgeGlobal for CustomSlotSample
impl TBridgeGlobal for CustomSlotSample {
  /// constructor
  fn void() -> Self {
    CustomSlotSample::new(0, 0, 0, 0)
  }
}

/// CustomSlotSample
impl CustomSlotSample {
  /// constructor
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    CustomSlotSample{r, g, b, a}
  }
}

/// minimum test
pub fn minimum_test() {
  // setup at main thread
  any_pinned_init_slots!(8);
  any_pinned_set_bg_mut!(bridge_global, 0);
  any_pinned_set_bg_mut!(CustomSlotSample, 1);

  // may be in another thread (bg: &mut bridge_global)
  any_pinned_with_bg_mut!(bridge_global, 0, |bg| {
    bg.num = 0;
  });

  // may be in another thread (bg: &mut CustomSlotSample)
  any_pinned_with_bg_mut!(CustomSlotSample, 1, |bg| {
    *bg = CustomSlotSample::new(240, 192, 32, 255);
  });

  // may be in another thread (bg: &mut bridge_global) test with move
  let local_value = 128i32;
  any_pinned_with_bg_mut!(bridge_global, 0, move |bg| {
    if local_value != bg.num as i32 {
      bg.num = local_value as usize;
      println!("value changed: local_value = {}", local_value);
    }
  });

  // may be in another thread (bg: &mut CustomSlotSample)
  any_pinned_with_bg_mut!(CustomSlotSample, 1, |bg| {
    println!("{:?}", bg);
  });

  // dispose at main thread
  any_pinned_dispose_slots!();
}
