fn main() {
  println!("{:?}", b"This will look like numbers"); // b -> bytes == ASCII codes

  /*
  println!("{}", b"This will look like numbers"); // error
  b"This will look like numbers" -> [u8; string.len()]
  [u8; string.len()] is printed on the screen not by Display, but Debug.
  Display, Debug is trait.
  Trait is something which types can implement.

  {:?} mean Debug
  */

  println!("{:X}", '행' as u32); // Cast Char as u32 to get the hexadecimal(16진수) value.
  println!("\u{D589}"); // Try printing it with unicode

  let number = 123;
  let number_ref = &number;
  println!("{:p}", &number_ref); // pointer
  println!("{}", number_ref);

  println!(
    "Binary : {:b}, hexadecimal: {:x}, octal: {:o}", // 2진수, 16진수, 8진수
    number, number, number
  )
}
