use rand::RngExt;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).expect("Failed to read line");


    let mut num : u32 = str.trim().parse().expect("Failed to parse");
    num += rand::rng().random_range(1..=100);

    println!("{}", num);
}
