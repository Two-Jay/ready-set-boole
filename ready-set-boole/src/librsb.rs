pub fn adder(left: u32, right:u32) -> u32 {
    let mut tmp_left = left;
    let mut tmp_right = right;
    for _ in 0..32 {
        let mut _carry = tmp_left & tmp_right;
        tmp_left = tmp_left ^ tmp_right;
        tmp_right = _carry << 1;
    }
    tmp_left
}

pub fn multiplier(left: u32, right:u32) -> u32 {
    let mut tmp_left = left;
    let mut tmp_right = right;
    let mut result = 0;
    for _ in 0..32 {
        if tmp_right & 1 != 0 {
            result = adder(result, tmp_left);
        }
        tmp_left <<= 1;
        tmp_right >>= 1;
    }
    result
}

pub fn gray_code(n : u32) -> u32 {
    let mut result = n ^ (n >> 1);
    result
}

pub fn eval_formula(formula : &str) -> bool {
    return true
}