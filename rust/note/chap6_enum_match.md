# 6. Enum and Patterns Matching
## 6.1. Enum
### 6.1.1. Enum
```rust
enum IpAddrKind {
  V4,
  V6,
}
struct IpAddr {
  kind: IpAddrKind,
  addr: String,
}

  let ip_addr_v4 = IpAddr{ kind: IpAddrKind::V4, addr: String::from("127.0.0.1") };
  let ip_addr_v6 = IpAddr{ kind: IpAddrKind::V6, addr: String::from("::1") };
```

### 6.1.2. Enum with Actual Value
```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String)
}

let ip_addr_v4 = IpAddr::V4(127, 0, 0, 1);
let ip_addr_v6 = IpAddr::V6(String::from("::1"));
```

### 6.1.3. The Option Enum and Over Null Values

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

```rust
enum Option<T> {
  None,
  Some(T),
}

  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
```

## 6.2. The `match` Control Flow
### 6.2.1.`match`
```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn coin_2_cent(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("it is a penny.");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

### 6.2.2. Patterns that Bind to values
```rust
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn coin_2_cent(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("it is a penny.");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    },
  }
}
  let cent_quarter = coin_2_cent(Coin::Quarter(UsState::Alaska));
```

### 6.2.3. Matching with Option<T>
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 6.2.4. The _ Placeholder

The `match` should be exhaustive in Rust, but it provides one placeholder to save the efforts to list all the possible values.
```rust
let some_u8_value = 0u8;
match some_u8_value {
  1 => println!("one"),
  3 => println!("three"),
  5 => println!("five"),
  7 => println!("seven"),
  _ => (),
}
```

### 6.3.5. `if let`
`if let` could behave the same as `match`, it saves some efforts to write the code, but missed the exclusive checking of `match`

```rust
  let mut count = 0;
  let coin = Coin::Quarter(UsState::Alaska);
  if let Coin::Quarter(state) = coin {
    println!("State quarter is from {:?}", state);
  } else {
    count += 1;
  }
```