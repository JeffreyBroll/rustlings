// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit=0,
    Echo=5,
    Move=888,
    ChangeColor=65536
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
