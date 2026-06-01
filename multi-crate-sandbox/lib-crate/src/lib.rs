pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(val: i32) -> i32 {
    val + 2
}

pub struct ImportantExcerpt<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        //&self.content
        let ret;
        {
            let temp = "temp";
            ret = temp;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
