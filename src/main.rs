use calculator::calculate;
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

    match calculate(first_number, operator, second_number) {
        Ok(result) => println!(
            "{} {} {} = {}",
            first_number, operator, second_number, result
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}
