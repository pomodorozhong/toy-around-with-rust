# Getting Started with Iced

Reference: [hecrj/iced: A cross-platform GUI library for Rust, inspired by Elm](https://github.com/hecrj/iced)

## To Run

```sh
git clone https://github.com/hecrj/iced
cd iced
cargo run --package counter
```

## To Build on Windows

+ Add a new line at the start of `iced/examples/counter/src/main.rs` :

```rust
#![windows_subsystem = "windows"]
```

+ Then

```sh
cd examples/counter
cargo build --release
```
