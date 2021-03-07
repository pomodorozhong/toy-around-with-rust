# Getting Started with Druid

Reference: [Get started with Druid - Druid](https://linebender.org/druid/get_started.html)

## Set up a Druid project

creating a new Rust project

```sh
> cargo new druid-example
```

And then adding Druid as a dependency to Cargo.toml

```toml
[dependencies]
druid = "0.7.0"
```

To show a minimal window with a label replace main.rs with this;

```rust
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}
```

## To Run

```sh
cargo run
```

## To Build on Windows

+ Add a new line at the start of `main.rs` :

```rust
#![windows_subsystem = "windows"]
```

+ Then

```sh
cargo build --release
```
