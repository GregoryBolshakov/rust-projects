use core::ops::Add;
use std::path::Display;
struct MyString {
    buf: Vec<u8>,
}

impl MyString {
    fn as_bytes(&self) -> &[u8] {
        return &self.buf.as_slice();
    }

    fn push_str(&mut self, other: &str) {
        self.buf.extend_from_slice(other.as_bytes());
    }

    fn new() -> MyString {
        MyString{buf: Vec::new()}
    }

    fn from(s: &str) -> MyString {
        let mut buf = Vec::new();
        buf.extend_from_slice(s.as_bytes());
        MyString{buf: buf}
    }
}

impl Add<&str> for MyString {
    fn add(mut self, rhs: &str) -> Self::Output {
        self.push_str(rhs);
        return self;
    }

    type Output = MyString;
}

fn main() {
   //let s1 
}
