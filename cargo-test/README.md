# cargo test

## Reference

+ [cargo test - The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
+ [Unit testing - Rust By Example](https://doc.rust-lang.org/nightly/rust-by-example/testing/unit_testing.html)
+ [How to Write Tests - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
+ [Dev-dependencies - Rust By Example](https://doc.rust-lang.org/nightly/rust-by-example/testing/dev_dependencies.html)

## Key takeaways

+ Cargo can run your tests with the `cargo test` command.
  + `$ cargo test` execute all the unit and integration tests of the current package.
  + `$ cargo test foo` will run any test with `foo` in its name.
+ Cargo looks for tests to run in two places: in each of your `src/` files and any tests in `tests/`.
+ Tests in your `src/` files should be unit tests, and tests in `tests/` should be integration-style tests.
+ To change a function into a test function, add `#[test]` on the line before `fn`.
  + When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the functions annotated with the `test` attribute and reports on whether each test function passes or fails.
+ The `#[cfg(test)]` annotation on a test module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.
+ Tests fail when something in the test function panics. There are some helper macros:
  + `assert!(expression)` - panics if expression evaluates to `false`.
  + `assert_eq!(left, right)` and `assert_ne!(left, right)` - testing left and right expressions for equality and inequality respectively.
+ To add dependencies for test only, add such dependencies to `Cargo.toml` in the `[dev-dependencies]` section. These dependencies are not propagated to other packages which depend on this package.

## The tests in here

To run both of the Unit test and the Integration test:

```sh
cargo test
```

### Unit test

`src/fizzbuzz_core.rs` and `src/fizzbuzz_wrapper.rs` demonstrate basic unit tests.

### Integration test

`tests/integrated.rs` demonstrate a basic integration test.
