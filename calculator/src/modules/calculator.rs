use num_traits::NumCast;
use std::io::Error;

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug)]
pub struct Calculator {
    operation: Operation,
    operator1: f64,
    operator2: f64,
    result: f64
}

impl Calculator {
    pub fn new(operation: Operation) -> Self {
        Calculator {
            operation,
            operator1: 0.0,
            operator2: 0.0,
            result: 0.0
        }
    }

    pub fn result(&mut self) {
        match self.operation {
            Operation::Add => {
                self.result = add(self.operator1,self.operator2);
            }
            Operation::Subtract => {
                self.result = subtract(self.operator1,self.operator2);
            }
            Operation::Multiply => {
                self.result = multiply(self.operator1,self.operator2);
            }
            Operation::Divide => {
                self.result = divide(self.operator1,self.operator2).expect("");
            }
        }
    }

    pub fn set_operator_1(&mut self, n: f64) {
        self.operator1 = n
    }

    pub fn set_operator_2(&mut self, n: f64) {
        self.operator2 = n
    }
}

fn add<T, U>(n1: T, n2: U) -> f64
    where
        T: NumCast,
        U: NumCast,
{
    let n1_f64: f64 = NumCast::from(n1).expect("Error trying to convert.");
    let n2_f64: f64 = NumCast::from(n2).expect("Error trying to convert.");

    n1_f64 + n2_f64
}

fn subtract<T, U>(n1: T, n2: U) -> f64
    where
        T: NumCast,
        U: NumCast
{
    let n1_f64: f64 = NumCast::from(n1).expect("Error trying to convert.");
    let n2_f64: f64 = NumCast::from(n2).expect("Error trying to convert.");

    n1_f64 - n2_f64
}

fn multiply<T, U>(n1: T, n2: U) -> f64
    where
        T: NumCast,
        U: NumCast
{
    let n1_f64: f64 = NumCast::from(n1).expect("Error trying to convert.");
    let n2_f64: f64 = NumCast::from(n2).expect("Error trying to convert.");

    n1_f64 * n2_f64
}

fn divide<T, U>(n1: T, n2: U) -> Result<f64, Error>
    where
        T: NumCast,
        U: NumCast
{
    let n1_f64: f64 = NumCast::from(n1).expect("Error trying to convert.");
    let n2_f64: f64 = NumCast::from(n2).expect("Error trying to convert.");

    if n2_f64 != 0.0 {
        Ok(n1_f64 / n2_f64)
    } else {
        Err(Error::other("You can't divide by 0."))
    }
}