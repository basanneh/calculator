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

    let cmd = match operator.as_str() {
        "+" => Command::Add(first_number, second_number),
        "-" => Command::Subtract(first_number, second_number),
        "*" => Command::Multiply(first_number, second_number),
        "/" => Command::Divide(first_number, second_number),
        "^" => Command::Pow(first_number, second_number),
        "%" => Command::Mod(first_number, second_number),
        _ => {
            eprintln!("Unsupported operator '{}'. Use +, -, *, /, ^, or %.", operator);
            return;
        }
    };

    match execute(cmd) {
        Ok(value) => println!("Result: {}", value),
        Err(e) => eprintln!("Error: {:?}", e),
    }

   }
