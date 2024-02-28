use std::io;

fn main() {
     println!("Enter the first number:");
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read line");
 
     let num1: f64 = input.trim().parse().expect("Invalid number");
 
     println!("Enter the operation (+, -, *, /):");
     input.clear();
     io::stdin().read_line(&mut input).expect("Failed to read line");
 
     let opr: char = input.trim().chars().next().expect("Invalid operation");
 
     println!("Enter the second number:");
     input.clear();
     io::stdin().read_line(&mut input).expect("Failed to read line");
 
     let num2: f64 = input.trim().parse().expect("Invalid number");
 
     let operation_enum = match opr {
         '+' => Operation::Add(num1, num2),
         '-' => Operation::Subtract(num1, num2),
         '*' => Operation::Multiply(num1, num2),
         '/' => Operation::Divide(num1, num2),
         _ => panic!("Invalid operation"),
     };
 
     let result = calculate(&operation_enum);
 
     println!("Result: {}", result);
}

enum Operation {
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(op: &Operation) -> f64 {
    match op {
        Operation::Add(x,y) => x+y,
        Operation::Subtract(x,y) => x-y,
        Operation::Multiply(x,y) => x*y,
        Operation::Divide(x,y) => x/y,
    }
}
