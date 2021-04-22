use std::io;

fn main(){
    println!("Guess the number");
    println!("Input your guess:");

    let mut guess = String::new();

    let foo = 5; //immutable
    let mut foo = 5; //mutable


    io::stdin().read_line(&mut guess). //std::io::stdin; & reference;
        expect("Error occurred."); // io::Result

    println!("You guessed:{}",guess);
}