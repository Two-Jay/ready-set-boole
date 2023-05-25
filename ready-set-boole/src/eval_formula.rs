use std::collections::VecDeque;
use std::collections::Vec;

fn get_F(&char ch, &VecDeque vd) -> i8 {
    vd.push_back(0);
}

fn get_T(&char ch, &VecDeque vd) -> i8 {
    vd.push_back(1);
}

fn init_state_behavior(&Vec behaviors) {
    behaviors[0] = get_F;
    behaviors[1] = get_T;
}

pub fn eval_formula(_formula : &str) -> bool {
    let mut vecdeque = new VecDeque();
    let mut state_behaviors = new Vec();

    for _ in str {
        
    }
    return true;
}