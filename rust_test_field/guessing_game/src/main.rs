extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

const HEADER : &str = "** !! Guessing the number !! **";
const INPUT_MESSAGE : &str = "Please input your guess.";
const INPUT_ERRMSG : &str = "Failed to read line";

fn print_header(message : &str) {
    println!("{}", message);
}

fn input_string_from_user() -> String {
    println!("{}",INPUT_MESSAGE);

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect(INPUT_ERRMSG);
    input.trim().to_string()
}

fn convert_string_to_int(user_input : String) -> Result<i8, std::num::ParseIntError> {
    user_input.parse::<i8>()
}

fn get_rand_secret(min : i8, max : i8) -> i8 {
    let ret : i8 = loop {
        let num = rand::thread_rng().gen_range(min, max);
        if i32::from(num) >= std::i8::MIN as i32 && i32::from(num) <= std::i8::MAX as i32 {
            break num as i8
        }
    };
    ret
}

fn main() {
    print_header(HEADER);
    let secret = get_rand_secret(0, 101);

    loop {
        let user_input = input_string_from_user();
        if user_input == "exit" {
            println!("{}", "exit!!");
            break;
        }
        let input = convert_string_to_int(user_input);
        match input {
            Ok(input) => println!("Your guess : {}", input),
            Err(error) => {
                println!("Error : {}", error);
                continue;
            }
        }
        match input.expect("REASON").cmp(&secret) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too greater!!"),
            Ordering::Equal => {
                println!("You got the secret! Bye!!");
                break 
            }
        }
    }
}
