pub fn is_fizzbuzz(n: i64) -> bool {
    n % 15 == 0
}
pub fn is_fizz(n: i64) -> bool {
    n % 3 == 0
}
pub fn is_buzz(n: i64) -> bool {
    n % 5 == 0
}

pub fn fizzbuzz_core(n: i64) -> (bool, bool, bool, i64) {
    (is_fizzbuzz(n), is_fizz(n), is_buzz(n), n)
}

#[cfg(test)]
mod fizzbuzz_core_tests {
    use super::*;

    #[test]
    fn fizzbuzz_core_15_true_true_true_15() {
        assert_eq!(fizzbuzz_core(15), (true, true, true, 15));
    }
    #[test]
    fn is_fizzbuzz_87_false() {
        assert_eq!(is_fizzbuzz(87), false);
    }
    #[test]
    fn is_fizzbuzz_15_true() {
        assert_eq!(is_fizzbuzz(15), true);
    }
    #[test]
    fn is_fizz_2_false() {
        assert_eq!(is_fizz(2), false);
    }
    #[test]
    fn is_fizz_3_true() {
        assert_eq!(is_fizz(3), true);
    }
    #[test]
    fn is_fizz_3_false() {
        assert_eq!(is_buzz(3), false);
    }
    #[test]
    fn is_fizz_5_true() {
        assert_eq!(is_buzz(5), true);
    }
}
