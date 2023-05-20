mod adder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_test_00() {
        let result = adder(2, 2);
        assert_eq!(result, 4);
    }

    fn plus_test_01() {
        let result = adder(10, 2);
        assert_eq!(result, 12);
    }

    fn plus_test_02() {
        let result = adder(100, 4);
        assert_eq!(result, 104);
    }

    fn plus_test_03() {
        let result = adder(3100, 123);
        assert_eq!(result, 3223);
    }

    fn zero_test_00() {
        let result = adder(0, 0);
        assert_eq!(result, 0);
    }

    fn zero_test_01() {
        let result = adder(0, -22);
        assert_eq!(result, -22);
    }

    fn zero_test_02() {
        let result = adder(0, 38);
        assert_eq!(result, 38);
    }

    fn minus_test_00() {
        let result = adder(-2, -2);
        assert_eq!(result, -4);
    }

    fn minus_test_01() {
        let result = adder(-12, -22);
        assert_eq!(result, -34);
    }

    fn minus_test_02() {
        let result = adder(-123, -38);
        assert_eq!(result, -161);
    }

    fn minus_test_03() {
        let result = adder(-123, 38);
        assert_eq!(result, -85);
    }
}
