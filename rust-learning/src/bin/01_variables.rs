// Exercise 01 — variables, mutability, shadowing.
// Run with: cargo run --bin 01_variables

fn main() {
    let immutable = 5;
    println!("immutable = {immutable}");

    let mut mutable = 10;
    println!("mutable before = {mutable}");
    mutable += 1;
    println!("mutable after  = {mutable}");

    let shadowed = "42";
    let shadowed: i32 = shadowed.parse().expect("not a number");
    println!("shadowed (now i32) = {shadowed}");

    const MAX_RETRIES: u32 = 3;
    println!("MAX_RETRIES = {MAX_RETRIES}");
}
