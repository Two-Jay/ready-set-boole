mod adder;
mod multiplier;
mod graycode;
mod eval_formula;

use adder::*;
use multiplier::*;
use graycode::*;
use eval_formula::*;

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



#[cfg(test)]
mod gray_code_test {
    use super::*;

    #[test]
    fn gray_testcase_00() {
        let param : u32 = 0;
        let answer : u32 = 0;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_01() {
        let param : u32 = 1;
        let answer : u32 = 1;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_02() {
        let param : u32 = 2;
        let answer : u32 = 3;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_03() {
        let param : u32 = 3;
        let answer : u32 = 2;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_04() {
        let param : u32 = 4;
        let answer : u32 = 6;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_05() {
        let param : u32 = 5;
        let answer : u32 = 7;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_06() {
        let param : u32 = 6;
        let answer : u32 = 5;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer)
    }

    #[test]
    fn gray_testcase_07() {
        let param : u32 = 7;
        let answer : u32 = 4;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer);
    }

    #[test]
    fn gray_testcase_08() {
        let param : u32 = 8;
        let answer : u32 = 12;
        let result: u32 = gray_code(param);
        assert_eq!(result, answer);
    }
}

#[cfg(test)]
mod eval_formula_test {
    use super::*;

    #[test]
    fn formula_testcase_00() {
        let param : &str = "10&";
        let answer : bool = false;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }

    #[test]
    fn formula_testcase_01() {
        let param : &str = "10|";
        let answer : bool = true;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }

    #[test]
    fn formula_testcase_02() {
        let param : &str = "11>";
        let answer : bool = true;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }

    #[test]
    fn formula_testcase_03() {
        let param : &str = "10=";
        let answer : bool = false;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }

    #[test]
    fn formula_testcase_04() {
        let param : &str = "1011||=";
        let answer : bool = true;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }

    #[test]
    fn formula_testcase_05() {
        let param : &str = "!10&";
        let answer : bool = false;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }


    #[test]
    fn formula_testcase_06() {
        let param : &str = "1!0&";
        let answer : bool = true;
        let result: bool = eval_formula(param);
        assert_eq!(result, answer);
    }
}
