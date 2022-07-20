#[allow(unused)]

fn main() {
  let number : u8 = 10;
  let same_number = 10u8; // 10u8 == 10 of type u8
  let same_number2 = 10_u8;

  let big_number = 100_000_000_i32;
  // underbar 는 컴파일러에 의해 무시된다.

  let float = 5.; // default f64, Rust sees . and knows that it is a float.
  let other_float : f32 = 5.4;

  //let result_float = float + other_float; //error mismatched types;
  let result = float + other_float as f64;

  //let a : u8 = 256; // overflow
}