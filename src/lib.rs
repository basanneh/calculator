pub enum Command {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Mod(f64, f64),
    Pow(f64, f64),
}

pub fn execute(cmd: Command) -> Result<f64, String> {
    match cmd {
        Command::Add(a, b) => Ok(a + b),
        Command::Subtract(a, b) => Ok(a - b),
        Command::Multiply(a, b) => Ok(a * b),
        Command::Pow(a, b) => Ok(a.powf(b)), //exponentiation
        Command::Mod(a, b) => {
            if b == 0.0 {
                Err("Zero cannot be the denominator".to_string())
            } else {
                Ok(a % b)
            }
        }
        Command::Divide(a, b) => {
            if b == 0.0 {
                Err("Zero cannot be the denominator".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}
