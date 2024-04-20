//! global bridge c-bindings of any type slot for Rust
//! anyslot integrates bindings to cppbridge
//!
//! - cc-rs https://crates.io/crates/cc
//! - bindgen https://crates.io/crates/bindgen
//!
//! # cc-rs
//!
//! - include/bridge.hpp
//! - src/bridge.cpp
//!
//! # bindgen
//!
//! from
//!
//!  - include/bridge.hpp
//!
//! to
//!
//!  - include/bridge_bindings.rs
//!

pub mod cppbridge;
pub use cppbridge::*;

pub use std::pin::Pin;
pub use std::sync::Arc;
pub use std::cell::RefCell;
pub use std::borrow::BorrowMut;

/// trait TBridgeGlobal
pub trait TBridgeGlobal {
  /// constructor
  fn void() -> Self;
}

/// TBridgeGlobal for bridge_global
impl TBridgeGlobal for bridge_global {
  /// constructor
  fn void() -> Self {
    bridge_global{num: 0, buf: [0; 8]}
  }
}

/// fake for any pinned type
#[macro_export]
macro_rules! any_pinned_init_slots {
  ($n: expr) => {
    unsafe {
      let r = bridge_global_init_slots($n);
      if r == 0 {
        panic!("not allocated area: bridge_global_init_slots");
      }
      r
    }
  }
}
pub use any_pinned_init_slots;

/// fake for any pinned type
#[macro_export]
macro_rules! any_pinned_dispose_slots {
  () => {
    unsafe {
      let r = bridge_global_dispose_slots();
      if r == 0 {
        panic!("something wrong to free: bridge_global_dispose_slots");
      }
      r
    }
  }
}
pub use any_pinned_dispose_slots;

/// fake for any pinned type
#[macro_export]
macro_rules! any_pinned_set_bg_mut {
  ($i: ty, $n: expr) => {
    let mut abg = Arc::new(RefCell::new(<$i>::void()));
    let mut pbg = Pin::new(&mut abg); // to keep lifetime
    pbg.set_bg_mut($n);
  }
}
pub use any_pinned_set_bg_mut;

/// fake with_bg_mut for any pinned type
#[macro_export]
macro_rules! any_pinned_with_bg_mut {
  ($i: ty, $n: expr, $f: expr) => {
    Pin::<&mut Arc<RefCell<$i>>>::with_bg_mut($n, $f)
  }
}
pub use any_pinned_with_bg_mut;

/// trait TGlobalSetGet
pub trait TGlobalSetGet<T> {
  /// set bg
  fn set_bg_mut(&mut self, n: usize);
  /// with bg
  fn with_bg_mut<F>(n: usize, f: F) where F: FnMut(&mut T) -> ();
}

/// TGlobalSetGet for Pin<&mut Arc<RefCell<T>>>
impl<T> TGlobalSetGet<T> for Pin<&mut Arc<RefCell<T>>> {
  /// set bg
  fn set_bg_mut(&mut self, n: usize) {
    unsafe {
      match bridge_global_setter(n, self.borrow_mut().as_ptr()
        as *mut T as *mut bridge_global) {
        0 => panic!("not allocated area: bridge_global_setter"),
        _ => ()
      }
    }
  }
  /// with bg
  fn with_bg_mut<F>(n: usize, mut f: F) where F: FnMut(&mut T) -> () {
    f(unsafe {
      let p = bridge_global_getter(n);
      if p == 0 as *mut bridge_global {
        panic!("not allocated area: bridge_global_getter");
      }
      &mut std::slice::from_raw_parts_mut(p as *mut T, 1)[0] // fake
    })
  }
}
