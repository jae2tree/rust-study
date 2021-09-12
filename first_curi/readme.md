# Easy Rust (https://dhghomon.github.io/easy_rust/)

## 08_types(명사와 동사)
### - 명사
#### 1. Primitive types - Data is in stack. 숫자, 문자(Car), Tuple ...
The signed integers are; `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.   
The unsigned integers are; `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. floating number `f16`, `f32`.   
usize is the best size for indexing.

>- Rust always chooses `i32` for integers if you don't tell it to use a different type
>- only `u8` can be cast as `char`
>- All chars use 4 bytes of memory
>- When using characters as part of a string, the string is encoded to use the least   
>- amount of memory needed for each character.

#### 2. Complex type - Data is in heap. 문자열(String) Struct, Vector...
### - 동사
Lifetime

-------------------
-------------------

## 09_type_interface (CPU와 메모리 vs Type과 Process)
Rust가 빠른 이유   
최적화 == Optimization => Resource(CPU, Memory)를 잘 활용한다.  
CPU 를 잘 활용(동사|시간를 이용함) => Process, Thread, Blocking, Non-Blocking, Asynchronous, Synchronous, Parralllel, Concurrency
Memory 를 잘 활용(명사 | 공간를 이용함) => Data Type, Lifetime(명사이기도 하고 동사이기도 함)

>In Rust, everything is data, and every data has its type
>- We designate the type of our variable, argument, constant, function...
>- Rust determines each type of each varialbe...   

>All types of our data in our codes are determined by compiler, we just >help the compiler.   
>We must follow the rules set by compiler. => fast and safe code.

------------------
-------------------
## 11_display_and_debug
Display, Debug => 계약의 일종(Trait)   
When a data type implement the specific trait, then the data can use the functionality which the trait provide.

