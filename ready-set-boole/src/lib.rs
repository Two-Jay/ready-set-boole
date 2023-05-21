mod librsb;

use librsb::adder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_test_00() {
        let result = adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn plus_test_01() {
        let result = adder(10, 2);
        assert_eq!(result, 12);
    }

    #[test]
    fn plus_test_02() {
        let result = adder(100, 4);
        assert_eq!(result, 104);
    }

    #[test]
    fn plus_test_03() {
        let result = adder(3100, 123);
        assert_eq!(result, 3223);
    }

    #[test]
    fn zero_test_00() {
        let result = adder(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn zero_test_02() {
        let result = adder(0, 38);
        assert_eq!(result, 38);
    }

    #[test]
    fn zero_test_03() {
        let result = adder(42, 0);
        assert_eq!(result, 42);
    }
}
