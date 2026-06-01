// struct Item {
//     name: String,
// }

// enum Message {
//     Quit,
//     Teleport(i32, i32, i32),
//     Say(String),
//     Give(Item),
// }

// impl Message {
//     fn send(&self) {
//         println!("Message sent");
//     }
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(String), // State
// }

// fn coin_to_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("The state of the quarter is {state}");
//             25
//         }
//     }
// }

// fn plus_one(opt: Option<i32>) -> Option<i32> {
//     match opt {
//         Some(val) => Some(val + 1),
//         None => None,
//     }
// }

// fn main() {
//     // let msgQuit = Message::Quit;
//     // let msgTeleport = Message::Teleport(5, 14, 128);
//     // let msgSay = Message::Say(String::from("Hello"));
//     // let msgGift = Message::Give(Item{name: String::from("Banana")});

//     // let opt: Option<i32> = Some(15);
//     // let opt2: Option<i32> = None;
//     // msgGift.send();

//     //coin_to_cents(Coin::Quarter(String::from("Alabama")));

//     let opt_five: Option<i32> = Some(5);
//     let opt_six = plus_one(opt_five);
//     let none = plus_one(None);
// }

//=================================================================

// use std::{thread, time::Duration};

// use rand::RngExt;

// fn give_fancy_hat(has_hat: &mut bool) {
//     match *has_hat {
//         false => {
//             *has_hat = true;
//             println!("Fancy hat given");
//         }
//         true => {
//             println!("Tried giving fancy hat, already had one");
//         }
//     }
// }

// fn remove_fancy_hat(has_hat: &mut bool) {
//     match *has_hat {
//         false => {
//             println!("Tried removing fancy hat, didn't have one");
//         }
//         true => {
//             *has_hat = false;
//             println!("Fancy hat removed");
//         }
//     }
// }

// fn move_n_tiles(n: u32) {
//     println!("Moved {n} tiles forward");
// }

// fn main() {
//     let rolls_remaining = 100;
//     let mut has_hat = false;

//     while rolls_remaining > 0 {
//         let roll1 = rand::rng().random_range(1..=6);
//         let roll2 = rand::rng().random_range(1..=6);
//         let roll_sum = roll1 + roll2;
//         match roll_sum {
//             3 => give_fancy_hat(&mut has_hat),
//             7 => remove_fancy_hat(&mut has_hat),
//             _ => move_n_tiles(roll_sum),
//         }

//         thread::sleep(Duration::from_millis(300));
//     }
// }

//=================================================================

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn main() {
    let coin = Coin::Quarter(String::from("Alaska"));

    if let Coin::Quarter(state) = coin {
        println!("The coin is a quarter with a state {state}");
    } else {
        println!("The coin is not a quarter");
    }
}