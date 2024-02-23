fn main() {
    let string1 = String::from("I am ");
    let string2 = String::from("Batman🦇");

    println!("{}",concatenate_strings(&string1,&string2));
}

fn concatenate_strings(s1: &str, s2: &str) -> String{
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}