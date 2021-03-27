use cargo_test::fizzbuzz_core::fizzbuzz_core;
use cargo_test::fizzbuzz_wrapper::fizzbuzz_wrapper;

#[test]
fn fizzbuzz_wrapper_fizzbuzz_core_15_fizzbuzz() {
    assert_eq!(
        fizzbuzz_wrapper(fizzbuzz_core(15)),
        String::from("fizzbuzz")
    );
}
#[test]
fn fizzbuzz_wrapper_fizzbuzz_core_3_fizzbuzz() {
    assert_eq!(fizzbuzz_wrapper(fizzbuzz_core(3)), String::from("fizz"));
}
#[test]
fn fizzbuzz_wrapper_fizzbuzz_core_4_fizzbuzz() {
    assert_eq!(fizzbuzz_wrapper(fizzbuzz_core(4)), String::from("4"));
}
#[test]
fn fizzbuzz_wrapper_fizzbuzz_core_5_fizzbuzz() {
    assert_eq!(fizzbuzz_wrapper(fizzbuzz_core(5)), String::from("buzz"));
}
