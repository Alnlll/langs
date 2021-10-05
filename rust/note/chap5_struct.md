# 5. Structure

## 5.1. Structure

- Statement
```rust
struct User {
  name: String,
  email: String,
  sign_in_count: u64,
  activate: bool
}
```

- Instance
```rust
  let user1 = User {
    email: String::from("www.example.com"),
    name: String::from("Rust"),
    activate: true,
    sign_in_count: 10,
  };
```

- Access and Modify
```rust
  let mut user2 = User {
    email: String::from("www.example.com"),
    name: String::from("Rust mut"),
    activate: true,
    sign_in_count: 10,
  };
  println!("user2: {:?}", user2);
  user2.name = String::from("John");
```

- Init from another instance
```rust
  let user3 = User {
    name: String::from("user3"),
    email: String::from("www.user3.com"),
    ..user1
  };
```

## 5.2. Tuple Structs without Named Fields

```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
```

## 5.3. Method Syntax

### 5.3.1. Defining Methods
```rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size}
  }
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

println!("area is {}", rect.area());
```

### 5.3.2. More Parameters
```rust
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
```

### 5.3.3. associated Functions

Rust allowed us to define functions without `self` as parameter within `impl` blocks, these are called `associated functions`. It could be called with `::`.

```rust
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size}
  }
}

let s = Rectangle::square(32);
```
