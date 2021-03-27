# cargo example

## Reference

+ [Karol Kuczmarski's Blog – Add examples to your Rust libraries](http://xion.io/post/code/rust-examples.html)
+ [Dev-dependencies - Rust By Example](https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html)

## Key takeaways

+ In Cargo’s parlance, an example is nothing else but a Rust source code of a standalone executable that typically resides in a single `.rs` file. All such files should be places in the `examples/` directory, at the same level as `src/` and the `Cargo.toml` manifest itself.
+ You can run an example through the typical `cargo run` command; simply pass the example name after the `--example` flag

```sh
$ cargo run --example hello
Hello from an example!
```

+ It is also possible to run the example with some additional arguments:

```sh
$ cargo run --example hello2 Alice
Hello, Alice!
```

+ To add dependencies for examples only, add such dependencies to `Cargo.toml` in the `[dev-dependencies]` section. These dependencies are not propagated to other packages which depend on this package.
+ When you run `cargo test`, all examples are built simultaneously with the execution of your regular test suite. This can be utilized to make sure all the examples still works.

## The examples in here

### hello.rs

`hello.rs` demonstrate a basic cargo example.

to run this example:

```sh
cargo run --example hello
```

---

### hello2.rs

`hello2.rs` demonstrate how to pass arguments to a cargo example.

to run this example:

```sh
cargo run --example hello2 your-arg-goes-here
```

---

### chrono.rs

`chrono.rs` demonstrate how to use `[dev-dependencies]` in a example.

to run this example:

```sh
cargo run --example chrono
```
