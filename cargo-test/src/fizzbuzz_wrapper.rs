pub fn fizzbuzz_wrapper((is_fizzbuzz, is_fizz, is_buzz, n): (bool, bool, bool, i64)) -> String {
    if is_fizzbuzz {
        String::from("fizzbuzz")
    } else if is_fizz {
        String::from("fizz")
    } else if is_buzz {
        String::from("buzz")
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod fizzbuzz_wrapper_tests {
    use super::*;

    #[test]
    fn fizzbuzz_wrapper_true_true_true_15_fizzbuzz() {
        assert_eq!(
            fizzbuzz_wrapper((true, true, true, 15)),
            String::from("fizzbuzz")
        );
    }
    #[test]
    fn fizzbuzz_wrapper_false_true_false_3_fizz() {
        assert_eq!(
            fizzbuzz_wrapper((false, true, false, 3)),
            String::from("fizz")
        );
    }
    #[test]
    fn fizzbuzz_wrapper_false_true_false_4_fizz() {
        assert_eq!(
            fizzbuzz_wrapper((false, false, false, 4)),
            String::from("4")
        );
    }
    #[test]
    fn fizzbuzz_wrapper_false_true_false_5_fizz() {
        assert_eq!(
            fizzbuzz_wrapper((false, false, true, 5)),
            String::from("buzz")
        );
    }
}
