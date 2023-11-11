# 0x02 Ecosystem
Category | Python | Rust
:-|:-|:-
Linter | pylint,pep8 | cargo clippy
LSP | python-language-server | RLS,rust-analyzer
code formatting | yapf,autopep8 | cargo fmt
build binary | setuptools,py2exe | cargo build
test | unittest, pytest | cargo test
build env | virtualenv,pip,requirments.txt | cargo new,cargo update, Cargo.toml
documentation | Sphinx based, doxygen | cargo doc
benchmark | cProfile | cargo bench, criterion.rs

- rustup
```bash
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /usr/local/rustup

1.72.0-x86_64-unknown-linux-gnu (default)
rustc 1.72.0 (5680fa18f 2023-08-23)
```

```bash
$ rustup component add clippy
info: downloading component 'clippy'
info: installing component 'clippy'
```

- starship
  - install
  ```bash
  curl -sS https://starship.rs/install.sh | sh
  # bash
  # Add the following to the end of ~/.bashrc:
  eval "$(starship init bash)"
  
  # fish
  # Add the following to the end of ~/.config/fish/config.fish:
  starship init fish | source

  # from_py_to_rust/01a_logging/logex on  master [$?] is 📦 v0.1.0 via 🦀 v1.72.0 
  # [Docker] ❯ 
  ```

- cargo
```bash
cargo build
cargo update
cargo run
cargo run --example demo
cargo doc
cargo test
```

- vscode
```
# Ctrl + Shift + p
# setting
# "rust-analyzer.linkedProjects": [
#        "path-to/listish/Cargo.toml"
#    ],
```

