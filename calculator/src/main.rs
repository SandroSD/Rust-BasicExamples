mod modules;

use modules::calculator::{Calculator, Operation};

use std::io::{self, Write};
use std::process;

fn save_user_input() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string() // trim returns a reference, so when the fn finish it's execution, the reference is going to point to something that will not exist anymore, that is why we need to_string()
}

fn main() {
    let string_input = r#"Please write the operation:
    - ADD (+)
    - SUBTRACT (-)
    - MULTIPLY (*)
    - DIVIDE (/)
    ----------------
    - EXIT (e/E)
    "#;

    println!("{}",string_input);

    let operation = save_user_input();
    println!("Operation selected: {}", operation);

    let mut calculator;

    if operation == "+" {
        calculator = Calculator::new(Operation::Add);
    } else if operation == "-" {
        calculator = Calculator::new(Operation::Subtract);
    } else if operation == "*" {
        calculator = Calculator::new(Operation::Multiply);
    } else if operation == "/" {
        calculator = Calculator::new(Operation::Divide);
    } else {
        println!("Finish execution without an operation.");
        process::exit(1);
    }

    println!("Write the first operator");
    let operator_input_1: f64 = save_user_input().parse().expect("Error");
    
    println!("Write the second operator");
    let operator_input_2: f64 = save_user_input().parse().expect("Error");

    calculator.set_operator_1(operator_input_1);
    calculator.set_operator_2(operator_input_2);

    calculator.result();

    println!("Calculator: {:#?}", calculator);

}