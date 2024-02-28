/*
    Traits are a powerful feature in Rust that allows you to define shared behavior for multiple types. 
    Think of them as a way to specify an interface that a type must adhere to. By using traits, 
    you can write functions and methods that can operate on different types, as long as those 
    types implement the traits you've defined. This encourages code reuse and modularity, 
    making your code easier to maintain and extend.

    Traits in Rust are similar to interfaces in languages like Java or C#. 
    They define a set of methods that a type must implement to satisfy the trait. 
    However, unlike interfaces, traits can also provide default implementations for 
    some or all of their methods, allowing for greater flexibility and code reuse.
*/

fn main() {
    let dog = Dog{
        name: String::from("Max"),
    };
    dog.speak();
}

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

trait Printable {
    fn print(&self);
}

// we can use this function to print any type that implements Printable trait
fn print_something<t: printable>(item: &T) {
    item.print();
}
</t: printable>
