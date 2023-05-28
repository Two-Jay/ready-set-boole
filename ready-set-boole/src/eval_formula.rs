use std::collections::VecDeque;

fn condition_get_false(value: &char, negation_flag: &bool) -> bool {
    return (*value == '1' && *negation_flag == true) || (*value == '0' && *negation_flag == false);
}

fn get_false(vd: &mut VecDeque<i8>) -> Option<f64> {
    vd.push_back(0);
    None
}

fn condition_get_true(value: &char, negation_flag: &bool) -> bool {
    return (*value == '0' && *negation_flag == true) || (*value == '1' && *negation_flag == false);
}

fn get_true(vd: &mut VecDeque<i8>) -> Option<f64> {
    vd.push_back(1);
    None
}

fn switch_negation_flag(flag: &mut bool) -> Option<f64> {
    *flag = if *flag == true { false } else { true };
    None
}

fn init_state_behavior(behaviors: &mut Vec<fn(&mut VecDeque<i8>) -> Option<f64>>) {
    behaviors.push(get_false);
    behaviors.push(get_true);
    behaviors.push(conjunction);
    behaviors.push(disjunction);
    behaviors.push(exclusive_disjunction);
}

fn conjunction(vd: &mut VecDeque<i8>) -> Option<f64> {
    if vd.len() < 2 {
        return None;
    }
    let a: i8 = vd.pop_back().unwrap();
    let b: i8 = vd.pop_back().unwrap();
    let result = if a & b == 1 { 1 } else { 0 };
    vd.push_front(result);
    None
}

fn disjunction(vd: &mut VecDeque<i8>) -> Option<f64> {
    if vd.len() < 2 {
        return None;
    }
    let a: i8 = vd.pop_back().unwrap();
    let b: i8 = vd.pop_back().unwrap();
    let result = if a == 1 || b == 1 { 1 } else { 0 };
    vd.push_front(result);
    None
}

fn exclusive_disjunction(vd: &mut VecDeque<i8>) -> Option<f64> {
    if vd.len() < 2 {
        return None;
    }
    let a: i8 = vd.pop_back().unwrap();
    let b: i8 = vd.pop_back().unwrap();
    let result = if a != b { 1 } else { 0 };
    vd.push_front(result);
    None
}

fn print_vd(vd: &mut VecDeque<i8>) -> Option<f64> {
    print!("vd state : [");
    for i in 0..vd.len() {
        if i == vd.len() - 1 {
            println!("{}]", vd[i]);
        } else {
            print!("{}, ", vd[i]);
        }
    }
    None
}

pub fn eval_formula(_formula: &str) -> bool {
    let mut stored: VecDeque<i8> = VecDeque::new();
    let mut state_behaviors: Vec<fn(&mut VecDeque<i8>) -> Option<f64>> = Vec::new();
    let mut _negation_flag: bool = false;

    init_state_behavior(&mut state_behaviors);
    let values = _formula.chars();
    for value in values {
        // println!("{}...........", value);
        if condition_get_false(&value, &_negation_flag) {
            state_behaviors[0](&mut stored);
            if _negation_flag == true {
                switch_negation_flag(&mut _negation_flag);
            }
            // println!("false value inserted");
        } else if condition_get_true(&value, &_negation_flag) {
            state_behaviors[1](&mut stored);
            if _negation_flag == true {
                switch_negation_flag(&mut _negation_flag);
            }
            // println!("false value inserted");
        } else if value == '!' {
            switch_negation_flag(&mut _negation_flag);
        } else if value == '&' {
            state_behaviors[2](&mut stored);
        } else if value == '|' {
            state_behaviors[3](&mut stored);
        } else if value == '^' {
            state_behaviors[4](&mut stored);
        }
        // print_vd(&mut stored);
        // println!("======")
    }
    return *stored.get(0).unwrap() == 1;
}
