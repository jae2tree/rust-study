#[allow(unused_variables)]

fn main() {
  println!("Hello,world!");
  // ! is Mecro -> is a code to generate some more codes.

  println!("Hello,world! nuber {} and {}", 8, 9);
  println!("{}", number());

  multiply(2, 4);
  println!("{}", multiply2(4, 6));

  {
    let block_number = 16;
  } // dropped block_number;

  // println!("{}", block_number); // error

  let temp_number = {
    let a = 8;
    a + 7
  };

  println!("{}", temp_number)
}

fn number() -> i32 {
  //8; // error ;이 있으면 리턴이 없다는 의미 (Statement)
  8
}

fn multiply(one: i32, two: i32) {
  let result = one * two;
  println!("{} * {} = {}", one, two, result);
}

fn multiply2(one: i32, two: i32) -> i32 {
  let result = one * two;
  println!("{} * {} = {}", one, two, result);
  result
}
