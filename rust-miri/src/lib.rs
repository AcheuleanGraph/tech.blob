mod nodes;
pub use nodes::*;


#[cfg(test)]
mod tests {

  #[test]
  fn no_memory_leak() {
    let x: *mut i32 = Box::into_raw(Box::new(0));
    unsafe {
      *x = 10;
      let _ = Box::from_raw(x);
    }
  }

  #[test]
  #[cfg_attr(miri, ignore)]
  fn yeah_memory_leak() {
    let x: *mut i32 = Box::into_raw(Box::new(0));
    unsafe {
      *x = 10;
    }
  }

  #[test]
  fn ok_stacked_borrow() {
    let mut a: i32 = 10;
    unsafe {
      let b: *mut i32 = &mut a;
      let c: *mut i32 = &mut *b;

      *c += 5;
      *b += 20;
    }
    assert_eq!(a, 35);
  }

  #[test]
  #[cfg_attr(miri, ignore)]
  fn bad_stacked_borrow() {
    let mut a: i32 = 10;
    unsafe {
      let b: *mut i32 = &mut a;
      let c: *mut i32 = &mut *b;

      *b += 5;
      *c += 20;
    }
    assert_eq!(a, 35);
  }
}