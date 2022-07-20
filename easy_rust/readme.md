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

## 12_Mutability
Mutable(변할 수 있는) <=> Immutable(변할 수 없는)
Rust is basically immutable style language.
data safety

## 13_The stack, heap and pointer
The size of values on stack are determined at compile time.
The size of values on heap are not determined at compile time.
Pointer can be applied both stack and heap data.

## 14_more_about_printing
in console log
[u8; 4] -> 4 values of witch type is u8.

## 16 Stack, Heap, Own, Borrow
Rust is Fast and Safe 

Fast -> CPU, Memory(Stack, Heap, Data Type)
Safe -> -> CPU, Memory(Own, Borrow)

Stack, heap -> Fast
Own, Borrow -> Safe

Own Data(name)
1. 모든 데이터는 어느 한 시점에 반드시 하나의, 꼭 하나 만의 소유자(Owner)를 갖는다. 그 소유자는 주로 Variables이다.
2. 모든 데이터는 태어나는 시점과 소멸하는 시점이 반드시 한 번씩, 꼭 한 번씩만 있다.
3. 모든 데이터의 소유권은 어느 한 소유자에게서 다른 소유자로 이전될 수 있다.
4. 모든 데이터는 특수한 상황에서 공동 소유자가 존재할 수 있다.

Borrow Data(&name or ref name)
1. 다른 소유자에게서 빌려온 데이터
2. 어느 한소유자는 자신의 데이터를 무한히 반복하여 빌려줄 수 있다.
3. 빌려주는 사람(소유자)는 반드시 빌려간 사람보다 오래 살아야 한다.
