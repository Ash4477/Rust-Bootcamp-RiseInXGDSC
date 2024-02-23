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

    // Data Types
    
    //integers can be of 8, 16, 32, 64, or 128 bits (signed or unsigned)
    let number_of_days: i8 = 7;
    let number_of_users: i64 = 12000;  // signed (+ or -)
    let number_of_tokens: u64 = 10000; // unsigned (just +)

    //float can be of 32 or 64 bits
    let pi: f32 = 3.14;

    // Char
    let _my_char = 'A';

    //Strings
    let msg: &str = "Hello";    // immutable

    let mut name = String::from("Alice");   // mutable

    // Arrays
    let numbers: [i32; 3] = [1,2,3];

    let second_number = numbers[1];
    println!("The second number in the array is {}.", second_number);

    // Slices
    let slice = &numbers[1..3];
    let first_element = slice[0];
    println!("The first element of the slice is {}.", first_element);

    // Tuples
    let person = ("Alice", 30);

    let name = person.0;
    let age = person.1;
    println!("The person's name is {} and their age is {}.", name, age);

    let person = (("Alice", "Smith"), 30);
    println!("The person's name is {} {} and their age is {}.", person.0.0, person.0.1, person.1);

}