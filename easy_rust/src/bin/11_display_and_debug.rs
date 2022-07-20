fn main() {
  let doesnt_print = (); // not primitive data type

  //println!("This will not print: {}", doesnt_print); // error
  println!("This will print: {:?}", doesnt_print);
}
