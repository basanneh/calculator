# CLI Calculator

A simple command-line calculator built in Rust as part of my Week 1 learning project.

## Features

- Basic arithmetic operations: addition, subtraction, multiplication, division, modulo and exponentiation
- Input validation and error handling
- Division by zero protection

## Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

## How to Run

1. Clone this repository
2. Navigate to the project directory
3. Run with cargo:
```bash
cargo run -- <number1> <operator> <number2>
```

## Examples
```bash
cargo run -- 10 "+" 5
# Output: 10 + 5 = 15

cargo run -- 20 "-" 8
# Output: 20 - 8 = 12

cargo run -- 6 "*" 7
# Output: 6 * 7 = 42

cargo run -- 15 "/" 3
# Output: 15 / 3 = 5
```

## Supported Operators

- `+` Addition
- `-` Subtraction
- `*` Multiplication (use quotes: `"*"`)
- `/` Division

## Learning Goals

This project helped me practice:
- Rust basics (variables, data types, functions)
- Command-line argument parsing
- Pattern matching with `match`
- Error handling
- Ownership and borrowing concepts

## Error Handling

The calculator handles:
- Invalid number of arguments
- Non-numeric inputs
- Division by zero
- Unsupported operators