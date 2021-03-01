# Getting Started with Iced

## To Run

```sh
git clone https://github.com/hecrj/iced
cd iced/examples/counter
cargo run --package counter
```

## To Build on Windows

+ add a new line to `iced/examples/counter/src/main.rs` :

```rust
#![windows_subsystem = "windows"]
```

+ Then `$cargo build`