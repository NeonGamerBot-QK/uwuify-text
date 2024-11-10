fn main() {
    let text = std::env::args().nth(1).expect("no pattern given");
    println!("Hello, world! {:?}", text);
}
