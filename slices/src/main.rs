fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();

    for i in 0..bytes.len() {
        if bytes[i] == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*fn assert_copy<T: Copy>() {}
fn checks() {
    assert_copy::<String>();
}*/

fn main() {
    println!("{}", first_word(&String::from("Hello, world")));

    let s1 = String::from("Something");
    let s2 = s1.clone();
}
