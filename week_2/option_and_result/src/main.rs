/*
Option and Result are already defined enums in Rust's library.
1) Option
    pub enum Option {
        Some(T),    //Some(T): Represents a value of type T
        None,       //None: Represents the absence of a value
    }
    
2) Result    
    pub enum Result {
        Ok(T),      //Ok(T): Represents a successful operation with a value of type T
        Err(E),     //Err(E): Represents an error with a value of type E
    }

*/

fn main() {
    // Option Example
    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is: {}", number, value),
        None => println!("The square root of {} is not a real number.", number),
    }

    // Result Example
    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);

    match division_result {
        Ok(value) => println!("{} divided by {} is: {}", a, b, value),
        Err(error_message) => println!("Error: {}", error_message),
    }

    // Using both
    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_triangle_area(base, height);

    match area_result {
        Ok(area) => println!("The area of the triangle is: {} square units.", area),
        Err(error_message) => println!("Error: {}", error_message),
    }
}

fn find_square_root(number: f64) -> Option {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result {
    if b == 0.0 {
        Err("Division by zero is not allowed.".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_from_database(key: &str) -> Option {
    let database = vec![
        ("base", Some(4.0)),
        ("height", Some(6.0)),
    ];

    for (k, v) in database {
        if k == key {
            return v;
        }
    }
    None
}

fn calculate_triangle_area(base: Option, height: Option) -> Result {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive numbers.".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        },
        (None, _) => Err("The base is missing.".to_string()),
        (_, None) => Err("The height is missing.".to_string()),
    }
}
