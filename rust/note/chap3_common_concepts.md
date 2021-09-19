# Common Programming Concepts

## 2.1. Variables and Mutability

### 2.1.1. Mutability

- Variables are immutable by default, we need to make it mutable if needed.
- We could not change the value if a variable is immutable and bound to a name.

### 2.1.2. Difference Between Variables and Constants
Constants uses <span style="color:orange">const</span> keyword instead of <span style="color:orange">mut</span> to declare. Constants could not use `mut` to make it mutable.

```rust
const MAX_POINTS: u32 = 100_000;
println!("MAX_POINTS = {}", MAX_POINTS);
MAX_POINTS = 203;


error[E0070]: invalid left-hand side of assignment
  --> src/main.rs:14:14
   |
14 |   MAX_POINTS = 203;
   |   ---------- ^
   |   |
   |   cannot assign to this expression

```

### 2.1.3. Shadowing
We could declare a new variable with the same name with a previous one, Rust says that the first variable is `shadowed` by the second one.

```rust
  let x = 5;
  let x = x + 1;
  let x = x + 2;

  println!("x = {}", x);
```

- By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
- The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

## 2.2. Data Types

Rust is a `statically` typed language, which means that it must know the types of each variables at the compile time.

The `data types` could be summarized to 2 kinds: `scalar` and `compound`.

### 2.2.1. Scalar Types

A scalar type represents a single value, Rust has 4 primary scalar types:
- integers
- floating-point
- Booleans
- characters

#### 2.2.1.1. Integers

Length | Signed | Unsigned |
| :- | :- | :- |
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
128-bit | i128 | u128
arch | isize | usize 

Each signed variant can store numbers from $-2^{n-1}$ to $2^{n-1}-1$ inclusive, where `n` is the number of bits that variant uses. 

Unsigned variants can store numbers from 0 to $2^n - 1$, so a `u8` can store numbers 0 to 255.

Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

Number Literals | Example
| :- | :- |
Decimal | 98_322, 98322
Hex | 0xff
Octal | 0o77
Binary | 0b1111_0000, 0b11110000
Byte(u8 only) | b'A'

#### 2.2.1.2. Floating Types
- f32
- f64

```rust
fn main() {
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
}
```

#### 2.2.1.3. Numeric Operations
- addition
- subtraction
- multiplication
- division
- remainder

```rust
fn main() {
  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;

  // remainder
  let remainder = 43 % 5;
}
```

#### 2.2.1.4. The Boolean Type
Boolean type in Rust has two possible values: `true` and `false`.

```rust
fn main() {
  let t = true;

  let f: bool = false; // with explicit type annotation
}
```

#### 2.2.1.5. The Character Type

Rust's `char` type is 4 bytes in size and represents a Unicode Scalar Value.

```rust
fn main() {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat: char = '😻';

  println!("{} {} {} ", c, z, heart_eyed_cat);
}
```

### 2.2.2. Compound Types
- Tuples
- Arrays

#### 2.2.2.1. Tuples
A tuple is a general way of grouping together **a number of values with a variety of types** into one compound type.

Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
  let tup: (char, i32, f64) = ('Z', 30, 34.56);
  println!("name: {}, age: {}, salary: {}", tup.0, tup.1, tup.2);
```

#### 2.2.2.2. Arrays

- Unlike a tuple, every element of an array **must have the same type**.
- Arrays in Rust are different from arrays in some other languages because arrays in Rust **have a fixed length**, like tuples.

```rust
  let a = [1, 2, 3, 4, 5];
//   let first = a[0];
//   let second = a[1];
  println!("a[0] = {}", a[0]);
```

### 2.2.3. Functions
```rust
fn plus_one(x: i32) -> i32 {
  x + 1
}

fn main() {
  let x = 1;
  println!("{}", plus_one(x));
}
```

### 2.2.4. Comments

### 2.2.4. Control Flow

#### 2.2.4.1. if

- if-else
```rust
  if 10 < number {
    println!("number {} > 10", number);
  } else {
    println!("number {} <= 10", number);
  }
```

- else if
```rust
  if 10 < number {
    println!("number {} > 10", number);
  } else if 10 == number {
    println!("number {} = 10", number);
  } else {
    println!("number {} < 10", number);
  }
```

- `if` in a let statement
```rust
  let condition = true;
  let x = if condition {5} else {10};
  println!("x = {}", x);
```

#### 2.2.4.2. Loops

- loop
```rust
  loop {
    println!("Again!")
  }
```
- while
```rust
  let mut i = 10;
  while 0 != i {
    i -= 1;
    println!("i = {}", i);
  }
  println!("Loop FINISHED.");
```

- for
```rust
  // container iter for
  let a = [10, 20, 30, 40, 50, 60, 70, 80];
  for elem in a.iter() {
    println!("val is {}.", elem);
  }

  // range for loop
  for i in {1..4}.rev() {
    println!("i = {}", i);
  }
```

