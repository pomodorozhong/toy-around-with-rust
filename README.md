# Toy around with rust

This is a gathering of personal notes.

## The basics

+ [Install Rust](https://www.rust-lang.org/tools/install)
+ [Creating a New Package - The Cargo Book](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)
  + `cargo new`: creates a new Cargo package in a new directory using the name you pass it
  + `cargo init`: creates a new Cargo package in current directory
+ [cargo example](./cargo-example/README.md)
+ [cargo test](./cargo-test/README.md)

## IDE

+ vscode
  + [rust-analyzer - Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
  + [Even Better TOML - Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

## GUI related

+ [A Survey of Rust GUI Libraries | boringcactus](https://www.boringcactus.com/2020/08/21/survey-of-rust-gui-libraries.html)
+ [Rust GUI: Introduction, a.k.a. the state of Rust GUI libraries (As of January 2021) - DEV Community](https://dev.to/davidedelpapa/rust-gui-introduction-a-k-a-the-state-of-rust-gui-libraries-as-of-january-2021-40gl)
+ [Getting Started with Iced](./gui-iced/Getting%20Started%20with%20Iced.md)
+ [Getting Started with Druid](./gui-druid/Getting%20Started%20with%20Druid.md)

## Rust users

+ [Rewriting the heart of our sync engine - Dropbox](https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine#-why-rewrite)
+ [Production users - Rust Programming Language](https://www.rust-lang.org/production/users)
+ [9 Companies That Use Rust in Production](https://serokell.io/blog/rust-companies)
+ [Game studio Ready At Dawn switching to Rust for all new development : programming](https://www.reddit.com/r/programming/comments/91fx7q/game_studio_ready_at_dawn_switching_to_rust_for/e2y4wgg/?utm_source=reddit&utm_medium=web2x&context=3)
  + "A survey at one point demonstrated that half of the most serious security bugs in Firefox were caused by memory unsafety."
  + "Rust's safety guarantees can let you be more aggressive, which feels counter intuitive at first... When you have a language that's checking stuff for you, you can be more confident that you're getting things correct."
  + "you never need to write a `Makefile`."
