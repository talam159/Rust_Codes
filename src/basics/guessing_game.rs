use std::io;

fn main(){
    println!("Guess the number");
    println!("Please input:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("your guess is {}", guess);
}