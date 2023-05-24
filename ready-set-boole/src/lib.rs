mod librsb;

use librsb::*;

#[cfg(test)]
mod adder_tests {
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

#[cfg(test)]
mod multiplier_tests {
    use super::*;
    
    #[test]
    fn times_test_00() {
        let a : u32 = 2;
        let b : u32 = 2;
        let result = multiplier(a,b);
        assert_eq!(result, a*b);
    }

    #[test]
    fn times_test_01() {
        let a : u32 = 42;
        let b : u32 = 4;
        let result = multiplier(a,b);
        assert_eq!(result, a*b);
    }

    #[test]
    fn times_test_02() {
        let a : u32 = 5;
        let b : u32 = 83;
        let result = multiplier(a,b);
        assert_eq!(result, a*b);
    }

    #[test]
    fn times_test_03() {
        let a : u32 = 293;
        let b : u32 = 32921;
        let result = multiplier(a,b);
        assert_eq!(result, a*b);
    }

    #[test]
    fn times_test_04() {
        let a : u32 = 283931903;
        let b : u32 = 0;
        let result = multiplier(a,b);
        assert_eq!(result, a*b);
    }
    
    #[test]
    fn times_test_gugudan() {
        for a in 1..10 {
            for b in 1..10 {
                let result = multiplier(a,b);
                assert_eq!(result, a*b);   
            }
        }
    }
}




