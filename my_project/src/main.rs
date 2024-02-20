fn main() {

    // Variables
    let msg = "Hello, world!";
    let x: i32 = 42;
    let pi: f64 = 3.14;
    let is_rust_fun: bool = true;
    let letter_a: char = 'a';

    // Functions
    // fn name(parameters) -> return_type {}
    fn add(x: i32, y: i32) -> i32 {
        return x + y;
        // OR
        x + y // no semicolon so, returns value
    }

    let x = 42; // variables can be redeclared (overwrites)

    if x >= 0 {
        println!("X is non-negative");
    }
    else {
        println!("x is negative");
    }

    let mut i = 1; // mutable variable

    while i<=5 {
        println!{"{}",i};   // {} is placeholder for i
        i += 1;
    }
}
