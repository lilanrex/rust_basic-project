use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
       println!("Guess the number!");
       let secret_number = rand::thread_rng().gen_range(1..=100);
       //println!("secret number is {secret_number}");
       loop { // to allow users to have multiple guesses
        println!("input your guess. ");
       let mut guess = String::new(); // variables are immutable by default, so we use mut to make it mutable
       io::stdin().read_line(&mut guess) // read_line is basically our input function
       .expect("Failed to read line");

       let guess: u32 = match guess.trim().parse() {
        Ok(num)=> num,
        Err(_) => continue,   
        }; // this because rust can't compare a string type and a number type,
       // so we need to convert the string(guess) into a number type
       println!("you guessed: {}" ,guess); // or {guess}
       
       match guess.cmp(&secret_number) {  // match is used to match the value/result from guess.cmp.... to the specified arms/conditions think of this as an advanced if/else statement
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too large"),
        Ordering::Equal => {
            println!("you win");
            break; //this breaks the loop 
        }
       }
       }
}