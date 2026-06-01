#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32, u32),
    Move(i32, i32, i32),
    Echo(&'static str),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize(5, 10));
    println!("{:?}", Message::Move(1, 2, 3));
    println!("{:?}", Message::Echo("Hello"));
    println!("{:?}", Message::ChangeColor(0, 255, 0));
    println!("{:?}", Message::Quit);
}
