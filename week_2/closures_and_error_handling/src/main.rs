use std::fs;

fn main() {
    // Closures are functions without names that can capture the enclosing environment. 
    // They are also known as anonymous functions or lambdas.
    
    // define a closure to print a text
    let print_text = || println!("Defining Closure");

    // call the closure
    print_text();

    // define a closure that takes an integer and returns a boolean
    let is_even = |x: i32| -> bool { x % 2 == 0 };

    // call the closure with different values
    println!("Is 2 even? {}", is_even(2)); // true
    println!("Is 3 even? {}", is_even(3)); // false

    /*

    The Magical Trio: Fn, FnMut, and FnOnce

    Now let's talk about Fn, FnMut, and FnOnce, the magical trio 
    of traits that let us use closures in Rust. What are they, exactly?

    FnOnce: This trait represents closures that can be called exactly once. 
    They may move (consume) values from their environment. It's like a one-time 
    party invitation – you use it, and it's gone.

    FnMut: This trait is for closures that can be called multiple times and can mutate 
    values from their environment. It's like having a key to the house – you can enter as many 
    times as you want, and you're allowed to move the furniture.

    Fn: This trait is for closures that can be called multiple times without mutating their 
    environment. It's like being a ghost – you can pass through the house as much as you want, 
    but you can't change anything.
       
    */

    // Error Handling
    let my_content = getFileContent("my_file.txt");
    match my_content {
        Ok(item) => println!(":> {}", item),
        Err(_) => println!(":<"),
    }
}

fn getFileContent(file_name: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

