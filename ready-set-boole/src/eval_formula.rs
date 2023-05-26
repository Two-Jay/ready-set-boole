use std::collections::VecDeque;

fn get_false(vd : &mut VecDeque<i8>) -> Option<f64> {
    vd.push_back(0);
    None
}

fn get_true(vd : &mut VecDeque<i8>) -> Option<f64> {
    vd.push_back(1);
    None
}

fn condition_get_true(value : &char, negation_flag : &bool) -> bool{
    return *value == '1' || (*value == '0' && *negation_flag == true);
}

fn condition_get_false(value : &char, negation_flag : &bool) -> bool{
    return *value == '0' || (*value == '1' && *negation_flag == true);
}

fn switch_negation_flag(flag : &mut bool) -> Option<f64> {
    *flag = if *flag == true  { false } else { true };
    None
}

fn init_state_behavior(behaviors : &mut Vec<fn(&mut VecDeque<i8>) -> Option<f64>>) {
    behaviors.push(get_false);
    behaviors.push(get_true);
    behaviors.push(conjunction);
}

fn conjunction(vd : &mut VecDeque<i8>) -> Option<f64> {
    // validate : is vd's len more than 2;
    let a : i8 = vd.pop_back().unwrap();
    let b : i8 = vd.pop_back().unwrap();
    let result = if a & b == 1 {1} else {0};
    vd.push_front(result);
    None
}

pub fn eval_formula(_formula : &str) -> bool {
    let mut stored : VecDeque<i8> = VecDeque::new();
    let mut state_behaviors : Vec<fn(&mut VecDeque<i8>) -> Option<f64>> = Vec::new();
    let mut _negation_flag : bool = false;

    init_state_behavior(&mut state_behaviors);
    let values = _formula.chars();
    for value in values {
        if condition_get_false(&value, &_negation_flag) {
            state_behaviors[0](&mut stored);
        } else if condition_get_true(&value, &_negation_flag) {
            state_behaviors[1](&mut stored);
        } else if value == '!' {
            switch_negation_flag(&mut _negation_flag);
        } else if value == '&' {
            state_behaviors[2](&mut stored);
        }
    }
    let result : bool = if *stored.get(0).unwrap() == 1 as i8 { true } else { false };
    return result;
}