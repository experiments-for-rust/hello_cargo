//number guessing game

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng()
        .gen_range(1,101);
    println!("[Debug] The secret number is: {}", secret_number);

    loop{
        println!("Input your guess:");

        let mut guess = String::new();

        //let foo = 5; //immutable
        //let mut foo = 5; //mutable

        io::stdin().read_line(&mut guess)
            .expect("Error occurred."); //io::Result
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YATTAZE");
                break;
            }
        }
    }
}