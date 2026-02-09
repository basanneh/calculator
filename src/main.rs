use std::env;

fn calculate(first_number: f64, operator: &str, second_number: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(first_number + second_number),
        "-" => Ok(first_number - second_number),
        "*" => Ok(first_number * second_number),
        "%" => Ok(first_number % second_number),
        "^" => Ok(first_number.powf(second_number)), //exponentiation
        "/" => {
            if second_number == 0.0 {
                Err("Division by zero is not allowed.".to_string())
            } else {
                Ok(first_number / second_number)
            }
        }
        _ => Err(format!(
            "Unsupported operator '{operator}'. Use +, -, *, /, %, or ^."
        )),
    }
}

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
