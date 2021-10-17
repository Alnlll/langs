# 7. Managing Growing Projects with Packages, Crates, and Modules

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## 7.1. Packages and Crates
A crate is a binary or library, the `crate root` is a source file that the Rust compiler starts from and makes up the root module of your crate.

A `package` is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes the how to build those crates.

If we were creating a package, it follows the below steps:

- `cargo new`: it gives a package folder
```shell
.
└── example
    ├── Cargo.toml
    └── src
        └── main.rs
```

- `Cargo.toml` is created for the package
- `src/main.rs` is the default `crate root` of a binary crate, while `src/lib.rs` is for lib crate
- A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.

## 7.2. Defining Modules to Control Scope and Privacy
Modules let us organize code within a crate into groups for readability and easy reuse. 

Modules also control the privacy of items, which is whether an item can be used by outside code `(public)` or is an internal implementation detail and not available for outside use `(private)`.

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
  }
}
```
Earlier, we mentioned that `src/main.rs` and `src/lib.rs` are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the `module tree`.

Notice that the entire module tree is rooted under the implicit module named `crate`.

```shell
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## 7.3. Paths for Referring to an Item in the Module Tree

### 7.3.1. Paths for Referring to an Item in the Module tree

If we want to call a function, we need to know its path, a path could be in two forms:
- An absolute path starts from a crate root by using a crate name or a literal `crate`
- A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // relative path
  front_of_house::hosting::add_to_waitlist();
}
```

`crate::front_of_house::hosting::add_to_waitlist();` we're using absolute path to call `add_to_waitlist`. The `add_to_waitlist` and `eat_at_restaurant` are at the same crate, so we can use `crate` keyword to start an absolute path.

`front_of_house::hosting::add_to_waitlist();` the second time we call `add_to_list` in `eat_at_restaurant` with relative path. The path starts from `front_of_house`, the name of the module defined at the same level of the module tree as `eat_at_restaurant`.

### 7.3.1. Exposing Paths with the pub Keyword
The previous code's path searching will be good, but it will fail to compile due to privacy issues.

Modules aren't useful only for organizing our code, they also define Rust's `privacy boundary`.

- The way privacy works in Rust is that all items(functions, methods, structs, enums, modules, and constants) are private by default.
- Items in a parent module can't use the private items inside child modules.
- Items in child modules can use the items in their ancestor modules.

Keyword `pub` is used to expose path to out of the module.

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // relative path
  front_of_house::hosting::add_to_waitlist();
}
```

### 7.3.3. Starting Relative Paths with `super`

`super` is like the `..` in filesystem path.

```rust
fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}
```

### 7.3.4. Making Structs and Enums Public
`pub` could be used to designate structs and enums as public, bu there are a few extra details.

`pub` only makes the struct public, but the struct's fields will still be private. We can make each field public or not on a case-by-case basis.

```rust
mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
}
```

## 7.4. Bring Paths Into Scope with the `use` keyword
### 7.3.1. Bringing Paths into Scope with the use Keyword

We could use keyword `use` to introduce the paths, also absolute path or relative path. It looks like the symbolic link in the filesystem.

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// use crate::front_of_house::hosting;
use self::front_of_house::hosting

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}
```

### 7.4.2. Creating Idiomatic use Paths

```rust
use self::front_of_house::hosting;
use self::front_of_house::hosting::add_to_waitlist();
```

Both of the 2 implementation accomplishes the work to use `add_to_waitlist`, but the first way to force us to specify the parent module name of function is a more idiomatic way to bring a function into scope with `use`.

The way could help us to distinguish local function and out of module function.

### 7.4.3. Providing New Names with the `as` Keyword

```rust
use self::front_of_house::hosting as FhouseHosting;
```

### 7.4.4. Re-exporting Names with pub use

When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`.

```rust
pub use crate::front_of_house::hosting;
```

### 7.4.5. Using External Packages
- Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.

- Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope.

### 7.4.6. Using Nested Paths to Clean Up Large use Lists

```rust
use std::{cmp::Ordering, io};
// use std::cmp::Ordering;
// use std::in;
use std::io::{self, Write};
// use std::io;
// use std::io::Write;
```

### 7.4.7. The Glob Operator
```rust
use std::collections::*;
```

## 7.5. Separating Modules into Different Files

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

// src/front_of_house.rs
pub mod hosting {
  pub fn add_to_waitlist() {}
}
```

We could also create file structure like `src/front_of_house/hosting.rs`.