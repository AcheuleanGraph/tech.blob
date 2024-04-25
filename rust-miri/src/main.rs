fn main() {
  let x: *mut i32 = Box::into_raw(Box::new(0));
  unsafe {
    let _ = Box::from_raw(x);

    let y = *x + 10;
  
    println!("{}", y);
  }


  let mut a: i32 = 10;
  let b = &mut a;
  *b += 20;
  a += 20;

  assert_eq!(a, 50);
}