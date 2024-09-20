fn main() {
    println!("Guess the number:");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("failed to read");

    print!("The number you guess is {input}.");
}
