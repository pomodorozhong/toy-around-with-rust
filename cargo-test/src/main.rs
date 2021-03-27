use cargo_test::fizzbuzz_core::fizzbuzz_core;
use cargo_test::fizzbuzz_wrapper::fizzbuzz_wrapper;

fn main() {
    for i in 1..16 {
        println!("{}", fizzbuzz_wrapper(fizzbuzz_core(i)));
    }
}
