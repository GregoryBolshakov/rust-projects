#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(len : u32) -> Rectangle {
        Self {width: len, height: len}
    }
}

fn main() {
    let rec = Rectangle {width: 40, height: 30 };
    println!("The area of the {rec:?} is {}", rec.area());

    let rec2 = Rectangle{width: 10, height: 20};
    match rec.can_fit(&rec2) {
        true => println!("Can fit"),
        false => println!("Cannot fit"),
    }

    println!("{:?}", Rectangle::square(5));
}
