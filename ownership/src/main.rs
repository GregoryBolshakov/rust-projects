struct Greeter {
    name : String,
}

impl Drop for Greeter {
    fn drop(&mut self) {
        println!("Goodbye, {}", self.name);
    }
}

fn takes_ownership(s : String) {
    println!("{s}");
}

fn gives(s : &str) -> String {
    String::from(s)
}

fn references_test() {
    let mut s : String = String::from("literal");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    println!("{r3}");
}

fn main() {
    {
        let greeter = Greeter{name : String::from("Bob")};
        println!("{}", greeter.name);
    }

    let s = String::from("My string");
    takes_ownership(s);

    println!("{}", gives("literal"));

    references_test();
}
