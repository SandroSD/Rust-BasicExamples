use std::io::{self, Write};
use std::process;

fn add(n1: i8, n2: i8) -> i8 {
    n1 + n2
}

fn subtract(n1: i8, n2: i8) -> i8 {
    n1 - n2
}

fn multiply(n1: i8, n2: i8) -> i8 {
    n1 * n2
}

fn divide(n1: i8, n2: i8) -> i8 {
    n1 / n2
}

fn main() {
    let mut input = String::new();

    let string_input = r#"Please write the operation:
    - ADD (+)
    - SUBTRACT (-)
    - MULTIPLY (*)
    - DIVIDE (/)
    ----------------
    - EXIT (e/E)
    "#;

    println!("{}", string_input);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let operator = input.trim();
    println!("You entered: {}", operator);

    let result: i8;

    if operator == "+" {
        result = add(1,2);
    } else if operator == "-" {
        result = subtract(127,2);
    } else if operator == "*" {
        result = multiply(9,3);
    } else if operator == "/" {
        result = divide(8,2);
    } else {
        println!("Finish execution without an operation.");
        process::exit(1);
    }

    println!("Result: {}", result);

}