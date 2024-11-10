fn main() {
    let text = std::env::args().nth(1).expect("No text to uwuify given");
    let endResult = text.chars()
    .map(|x| match x { 
        // 'll' => "www",
        'l' => "w",
        _ => x
    }).collect();
    println!("{}", endResult);
}
