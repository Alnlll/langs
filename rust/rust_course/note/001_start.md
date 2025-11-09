# https://course.rs/first-try/cargo.html

# Install
# Cargo
```bash
cargo new hello_world
cargo run
cargo build
cargo run --release
cargo build --release
cargo check
```
# Dependency Accerleration
## proxy
```bash
export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7891
```
## image repo
```toml
# $HOME/.cargo/config.toml
[registries]
ustc = { index = "https://mirrors.ustc.edu.cn/crates.io-index/" }

# when import dependency
[dependencies]
time = {  registry = "ustc" }

# other repo
# =================================
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

# 稀疏索引，要求 cargo >= 1.68
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
# =================================
```
## gloabl hub change
```toml
# $HOME/.cargo/config.toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```
