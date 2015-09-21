use std::io;

fn main() {
    println!("Guess a Number from 1 to 100!");
   
    let mut guess = String::new();

    io::stdin().read_line(&mut guess) 
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);     
}
