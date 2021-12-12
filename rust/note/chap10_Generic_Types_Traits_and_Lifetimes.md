# 10. Generic Types, Traits, and Lifetimes

## 10.1. Generic Data types

## 10.1.1. In Function Definitions

When defining a function that uses generics, we place the generics in the signature of the function where we would usually **specify the data types of the parameters and return value**.

```rust
fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}
```

But for this we will fall into error like the below:
```shell
   Compiling generic v0.1.0 (/home/aln/projects/onedrive/003_basic/002_langs/langs/rust/code/chap10/generic)
error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src/main.rs:29:13
   |
29 |     if item > largest {
   |        ---- ^ ------- T
   |        |
   |        T
   |
help: consider restricting type parameter `T`
   |
25 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
   |             ^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0369`.
error: could not compile `generic` due to previous error
```

The note mentions std::cmp::PartialOrd, which is a trait. Because we want to compare values of type T in the body, we can only use types whose values can be ordered. To enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that you can implement on types (see Appendix C for more on this trait).

### 10.1.2. In Struct Definition


```rust
  struct Point<T> {
    x: T,
    y: T,
  }

  struct Point<T, U> {
    x: T,
    y: U,
  }

  // generic type struct
  {
    let integer = Point {x: 5, y:10 };
    let float = Point {x: 1.0, y: 4.0};
    let mix = Point {x:4, y: 4.0 };

    println!("integer: x= {}, y = {}", integer.x, integer.y);
    println!("float: x= {}, y = {}", float.x, float.y);
  }
```

### 10.1.3. In Enum Definition
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
  Ok(T),
  Err(E)
}
```

### 10.1.4. In Method Definition

We could implement `struct` or `enum` with generic types, <T> follows the keywords.

```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
```

We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.

```rust
// Now only `f32` has implement of dis_from_origin
impl Point<f32> {
  fn dis_from_origin(&self) -> f32 {
    return (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

### 10.1.5. Performance of Code Using Generics
The code uses generics types will not run any slower than the one uses concrete types. Rust accomplish this with `Monomorphization` which is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
