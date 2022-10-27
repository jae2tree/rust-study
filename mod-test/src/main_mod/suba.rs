use crate::main_mod::subb;

use subc;

pub fn test() {
  println!("suba");
  subb::test();
  subc::test();
}

pub mod suba_nested {
  pub fn test() {
    println!("suba_nested");
    super::test();
  }
}
