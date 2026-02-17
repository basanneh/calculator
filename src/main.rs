use calculator::Command;
use calculator::execute;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <number1> <operator> <number2>");
        return;
    }

    let first_number: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number.", args[1]);
            return;
        }
    };
    let operator = &args[2];
    let second_number: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number.", args[3]);
            return;
        }
    };

    match operator.as_str() {
        "+" => {
            let result = execute(Command::Add(first_number, second_number));
            match result {
                Ok(value) => println!("Result: {}", value),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "-" => {
            let result = execute(Command::Subtract(first_number, second_number));
            match result {
                Ok(value) => println!("Result: {}", value),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "*" => {
            let result = execute(Command::Multiply(first_number, second_number));
            match result {
                Ok(value) => println!("Result: {}", value),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "/" => {
            let result = execute(Command::Divide(first_number, second_number));
            match result {
                Ok(value) => println!("Result: {}", value),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "%" => {
            let result = execute(Command::Mod(first_number, second_number));
            match result {
                Ok(value) => println!("Result: {}", value),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        _ => eprintln!("Unsupported operator '{}'. Use +, -, *, /, or %.", operator),
    };
}
