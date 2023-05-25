use std::collections::VecDeque;

fn get_false(vd : &mut VecDeque<i8>) -> Option<f64> {
    vd.push_back(0);
    None
}

fn get_true(vd : &mut VecDeque<i8>) -> Option<f64> {
    vd.push_back(1);
    None
}

fn init_state_behavior(behaviors : &mut Vec<fn(&mut VecDeque<i8>) -> Option<f64>>) {
    behaviors.push(get_false);
    behaviors.push(get_true);
}

pub fn eval_formula(_formula : &str) -> bool {
    let mut stored : VecDeque<i8> = VecDeque::new();
    let mut state_behaviors : Vec<fn(&mut VecDeque<i8>) -> Option<f64>> = Vec::new();

    init_state_behavior(&mut state_behaviors);
    let values = _formula.chars();
    for value in values {
        if value == '0' {
            state_behaviors[0](&mut stored);
        }
        if value == '1' {
            state_behaviors[1](&mut stored);
        }
    }
    return true;
}