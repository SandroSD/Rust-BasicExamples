use std::io::{self, Write};
use std::process;

fn add(n1: i16, n2: i16) -> i16 {
    n1 + n2
}

fn subtract(n1: i16, n2: i16) -> i16 {
    n1 - n2
}

fn multiply(n1: i16, n2: i16) -> i16 {
    n1 * n2
}

fn divide(n1: i16, n2: i16) -> i16 {
    n1 / n2
}

#[derive(Debug)]
struct Calculator {
    operation: String,
    /*Operator1 (i16),
    Operator2 (i16),*/
    result: i16
}

impl Calculator {
    fn new(operation: String) -> Self {
        Calculator {
            operation,
            result: 0
        }
    }

    fn result(&mut self) {
        match self.operation.as_str() {
            "+" => {
                self.result = add(1,2);
            }
            "-" => {
                self.result = subtract(127,2);                        
            }
            "*" => {
                self.result = multiply(9,3);
            }
            "/" => {
                self.result = divide(8,2);
            }
            _ => {
                println!("Wrong...");
            }
        }
    }
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

    println!("Operation selected: {}", string_input);

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    let operator = input.trim();

    let mut calculator;

    if operator == "+" {
        calculator = Calculator::new(String::from("+"));
    } else if operator == "-" {
        calculator = Calculator::new(String::from("-"));
    } else if operator == "*" {
        calculator = Calculator::new(String::from("*"));
    } else if operator == "/" {
        calculator = Calculator::new(String::from("/"));
    } else {
        println!("Finish execution without an operation.");
        process::exit(1);
    }

    calculator.result();

    println!("Calculator: {:#?}", calculator);

}