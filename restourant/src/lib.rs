#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    mod front_of_house;

    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn my_test() {
        front_of_house::frontdesk::meet_new_customer();

        use front_of_house::service;

        service::take_an_order();
    }

    #[test]
    fn my_test_hash_map() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(10, 15);
        map.insert(3, 7);
        println!("Test value is {}", map.get(&3).unwrap_or(&-1));
    }
}
