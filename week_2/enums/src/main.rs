fn main() {
    let current_weather = Weather::Sunny;

    let msg = Message::Write(String::from("Hello, Rust!"));

    process_message(msg);

    let my_color = Message::ChangeColor(122,34,222);

    if let Message::ChangeColor(r,g,b) = my_color {
        println!("My colors are valid ;>");
    } 
    else{
        println!("My colors are not valid :<")
    }

    my_color.call();

}


enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        match self {
            Message::Quit => println!("Quit"),
            Message::Write(t) => println!("Write {}",t),
            Message::Move{x,y} => println!("Move to x: {}, y: {}",x,y),
            Message::ChangeColor(r,g,b) => println!("Color: r: {}, g: {}, b: {}",r,g,b),
            // you get the idea
        }
    }   
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data.");
        }
        Message::Move {x,y} => {
            println!("Move to co-ordinates x: {} and y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {} blue: {} green: {}", r, g, b);
        }
    }
}
