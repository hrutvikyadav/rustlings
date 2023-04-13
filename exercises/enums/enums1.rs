// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move {x:u32, y:u32},
    ChangeColor(u32, u32, u32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("something".to_string()));
    println!("{:?}", Message::Move {x:1, y:2});
    println!("{:?}", Message::ChangeColor(0, 0, 0));
}
