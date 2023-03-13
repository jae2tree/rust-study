#[allow(unused)]
mod suba;

#[allow(unused)]
pub mod subb;

pub use subb::subb_nested;

pub fn test() {
  suba::test();
}

pub mod main_a {
  pub fn test() {
    println!("main_a");
    use super::subb;
    subb::test();
  }
}
