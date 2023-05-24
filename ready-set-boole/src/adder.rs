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