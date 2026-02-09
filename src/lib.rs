pub fn calculate(first_number: f64, operator: &str, second_number: f64) -> Result<f64, String> {
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
