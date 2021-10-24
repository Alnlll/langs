# 9.Error Handling

Rust groups errors into 2 major categories: `recoverable` and `unrecoverable`. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Unlike other languages, Rust has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error. 

## 9.1. Unrecoverable Errors with panic!

```rust
  panic!("crash and burn");
```

Use `RUST_BACKTRACE=1` to dump backtrace, it needs the program built with debug symbols that is the default behavior.

## 9.2. Recoverable Errors with Result

### 9.2.1. Recoverable Errors with Result
Most errors aren't serious enough to require the program so panic, we could use `Reuslt<T, E>` to handle such errors by matching.

```rust
enum Result<T, E> {
  Ok(Y),
  Err(E),
}
```

The `T` and `E` are generic type parameters, what we need to know now are:

- `T` represents the type of the value that will be returned in a success case.
- `E` representsthe type of the value that will be returned in a failure case.

```rust
    use std::fs::File;
    let f = match File::open("hello.txt") {
      Ok(file) => file,
      Err(error) => panic!("failed to open file: {:?}", error),
    };
```

### 9.2.2. Matching on Different Errors
The type of the value that `File::open` returns inside the `Err` is `io::Error`, it has a method `kind` that we can call to get an `io::ErrorKind` value. Then we could continue to handle different kind of errors.

```rust
  {
    use std::fs::File;
    use std::io::ErrorKind;

    let f = match File::open("hello.txt") {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("failed to create file: {:?}", e),
        }
        other_error => {
          panic!("failed to open file: {:?}", other_error)
        }
      }
    };
  }
```

### 9.2.3. Shortcuts for Panic on Error: unwrap and expect
`Result<T, E>` has some helper methods to help us to simplify various tasks. 

`unwrap` works like the `match`, it returns the `Ok(T)` or `Err(E)` to us. `unwrap` will also call panic for us. 
```rust
    use std::fs::File;
    let f = File::open("hello.txt").unwrap();

//aln@aln-T5-SKYLAKE:~/projects/onedrive/003_basic/002_langs/langs/rust/code/chap9/error$ cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//      Running `target/debug/error`
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:37:37
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`expect` will give us chances to choose error message, others are just like `unwrap`.
```rust
  let f = File::open("hello.txt").expect("failed to open hello.txt");
```

### 9.2.4. Propagating Errors

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 9.3.5. A Shortcut for Propagating Errors: the ? Operator

Rust provided an simple magic `?` to implement the same thing like previous match method.

Error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function

The `?` operator can be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression we defined in Listing 9-6. The part of the match that requires a return type of Result is return Err(e), so the return type of the function has to be a Result to be compatible with this return.

```rust
  {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
      let mut f = File::open("hello.txt")?;
      let mut s = String::new();
      f.read_to_string(&mut s)?;
      Ok(s)
    }
  }

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

## 9.3. To panic or not

Basically, we should give the chance to get recoverable error to the caller of our implementation, panic is more sutable for the below situations

- Examples, Prototype Code, and Tests
- Cases in Which You Have More Information Than the Compiler
- Guidelines for Error Handling
  - The bad state is not something that’s expected to happen occasionally.
  - Your code after this point needs to rely on not being in this bad state.
  - There’s not a good way to encode this information in the types you use.
- Creating Custom Types for Validation

