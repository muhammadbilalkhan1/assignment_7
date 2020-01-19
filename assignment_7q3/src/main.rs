use std::io;
use rand::Rng;
mod lib;
fn main() {
    lib::buy_book_in_bookshop();

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);


}
