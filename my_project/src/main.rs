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

    // Control Flow

    // while loop
    let mut i = 0;
    while i <= 5{
        println!("Counter value is {}.",i);
        i += 1;
    }

    // for loop
    for i in 1..5{
        println!("Num is {}",i);
    }

    for i in 1..=5{
        println!("Num is {}",i);
    }

    // loop
    i = 0;
    loop {
        println!("Counter Value is {}", i);
        i += 1;

        if (i > 6){
            break;
        }
    }

    // match

    let num = 5;

    match num {
        1 => {
            println!("This is first match arm!");
            println!("Woah");
        }

        3 => println!("Oh"),
        
        5 => println!("DAMN"),

        _ => println!("Number is something else"),
    }

    let result = match num {
        1 => "Num 1",
        2 => "Num 1",
        3 => "Num 1",
    }

    // Ownership Concept

    let s1 = String::from("Hello"); // s1 is a stack pointing to heap where hello is stored
    let s2 = s1;    // instead of copying s1 to s2, the ownership of s1's value is moved to s2
                    // and s1 is discarded. This is ownership concept. We cant use s1 values anymore.

    // Borrowing and References
    // There are two type of references in rust: immutable and mutable
    
    /*
    1) immutable (allow you to read but not modify)
        -> One important thing to note about immutable references is 
        that you can have multiple immutable references to the same value at the same time. 
        This is known as "aliasing", and it allows you to read the value from different parts 
        of your code without causing any issues.

        IMP -> However, you can't have a mutable reference and an immutable reference to the 
        same value at the same time, as this would violate Rust's borrowing rules.
    */

    let my_string = String::from("Hey");
    let my_ref = &my_string; // we use & to reference or borrow

    fn print_string(s: &String){ // referencing parameter
        println!("{}",s);
    }

    /*
    2) mutable (allow to read and modify) and
    */

    let r3 = &mut my_string; // mutable reference
}
