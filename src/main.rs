use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("[Debug] The secret number is: {}", secret_number);

    println!("Input your guess:");

    let mut guess = String::new();

    //let foo = 5; //immutable
    //let mut foo = 5; //mutable


    io::stdin().read_line(&mut guess). //std::io::stdin; & reference;
        expect("Error occurred."); // io::Result

    println!("You guessed:{}",guess);
}