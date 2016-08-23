#![allow(dead_code)]

fn main() {
  println!("{:?}", unsafe { hello_world(99) });
}

extern {
  fn hello_world(b: u8) -> u8;
}

#[no_mangle]
pub extern "C" fn rust_code(b: u8) {
  println!("rust ran this: {}", b);
}
