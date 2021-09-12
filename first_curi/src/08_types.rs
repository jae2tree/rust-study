#[allow(unused_variables)]

//Rust always chooses i32 for integers if you don't tell it to use a different type
/* fn main() {
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '😺';
} */


//only `u`8` can be cast as `char`
/*
fn main() {
    let my_number = 100; // We didn't write a type of integer,
                         // so Rust chooses i32. Rust always
                         // chooses i32 for integers if you don't
                         // tell it to use a different type
    println!("{}", my_number as char); // ⚠️
}
*/
/*
fn main() {
    let my_number = 100;
    println!("{}", my_number as u8 as char);
}
*/
/*
fn main() {
    let my_number: u8 = 100; //  change my_number to my_number: u8
    println!("{}", my_number as char);
}
*/


//All chars use 4 bytes of memory
//When using characters as part of a string, the string is encoded to use the least amount of memory needed for each character.

fn main() {
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}


/* std::mem::size_of::<char>() 의 의미
std -> Standard Libraary 
:: -> dirctory
mem -> file -> Module
::
size_of -> function(now) or Data Type or Constant
::      Function::<char> => Function::<Function Generics>
<cahr>
()
*/