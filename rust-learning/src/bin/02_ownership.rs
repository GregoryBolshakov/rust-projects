// Exercise 02 — ownership, borrowing, references.
// Run with: cargo run --bin 02_ownership

fn main() {
    let owned = String::from("hello");
    let length = compute_length(&owned);
    println!("'{owned}' has length {length}");

    let mut s = String::from("foo");
    push_bar(&mut s);
    println!("after push_bar: {s}");

    let moved = String::from("moved value");
    consume(moved);
    // println!("{moved}"); // Uncomment to see the borrow checker complain.
}

fn compute_length(s: &str) -> usize {
    s.len()
}

fn push_bar(s: &mut String) {
    s.push_str("bar");
}

fn consume(s: String) {
    println!("consumed: {s}");
}
