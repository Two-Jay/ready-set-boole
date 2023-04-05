use std::io;

const HEADER : &str = "** !! Guessing the number !! **";
const INPUT_MESSAGE : &str = "Please input your guess.";
const INPUT_ERRMSG : &str = "Failed to read line";
const CONVERT_ERRMSG : &str = "Failed to cast input to int";

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

fn convert_string_to_int(user_input : String) -> Result<i8, std::io::Error> {
    let mut ret = user_input.parse::<i8>()?;
    Ok(ret)
}

fn main() {
    print_header(HEADER);

    loop {
        let user_input = input_string_from_user();
        if user_input == "exit" {
            println!("{}", "exit!!");
            break;
        }
        let input = convert_string_to_int(user_input);
        match input {
            Ok(input) => println!("input : {}", input),
            Err(error) => println!("Error : {}", error),
        }
    }
}
