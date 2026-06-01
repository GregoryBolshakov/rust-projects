// use crate::private_stuff::Seal;

// mod private_stuff {
//     use std::fmt::Debug;

//     pub trait Seal: std::fmt::Display {}

//     impl Seal for i32 {}
//     impl Seal for i64 {}
//     impl Seal for String {}

//     //pub trait Numeric: Seal {}
// }

// //trait Numeric: private_stuff::Seal {}
// // How to implement all stuff required by Seal for Numberic?

// fn process_only_i32_i64_string<T: Seal>(val: T) {
//     println!("{val}");
// }

trait Summary {
    fn summarize(&self) -> String;
}

trait Summary2 {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

impl Summary2 for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

fn main() {
    let article = Article{title: String::from("Some title"), content: String::from("Some content")};
    Summary::summarize(&article);
}
