# 1. Getting started

- Installing rustup
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- Updating and Uninstalling
```bash
rustup update
rustup self uninstall
```

- version
```bash
rustc --version
```

## 1.1. Building Project without Cargo
```bash
$ mkdir hello_world
$ cd hello_world

# write code in hello.rs
$ rustc hello.rs
$ ./hello
Hello, world!
```

## 1.1. Building Project with Cargo

Cargo is Rust's build system and package management.

- Cargo Version
```bash
cargo --version
```

- new Project
```bash
cargo new hello_cargo

# with git repo, connect to git repo, won't create new git repo
cargo new --vcs=git hello_cargo
```

- Cargo.toml
```toml
# rust/code/hello_cargo/Cargo.toml
[package]
# is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.
name = "hello_cargo"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# is the start of a section for you to list any of your project’s dependencies.

```

- Building and Run
```bash
$ cd hello_cargo
$ cargo build
$ cargo run

Compiling hello_cargo v0.1.0 (/home/aln/projects/onedrive/003_basic/002_langs/langs/rust/code/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo is running the exe in **"target/debug/hello_cargo"**.

- We can build a project using **cargo build**.
- We can build and run a project in one step using **cargo run**.
- We can build a project without producing a binary to check for errors using **cargo check**.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the **target/debug directory**.

- For Release
```bash
cargo build --releases
```



