// calculator.rs
pub enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Division(f64, f64),
}

pub fn calculate(opr: Operation) -> f64 {
    match opr {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Division(x, y) => {
            if y == 0.0 {
                panic!("Division by zero is not allowed");
            }
            x / y
        }
    }
}
