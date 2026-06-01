use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();

    map.insert(123, String::from("123 string"));
    map.entry(223).or_insert(String::from("223 string"));
}
