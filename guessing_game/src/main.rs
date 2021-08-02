extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secrete Numbers : {}", secret_number);

    loop {


    println!("Please Input your guess : ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to Read Line");

        let guess: u32 = guess.trim().parse().expect("Fail to read Line");
    
        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Smaller!"), 
        Ordering::Greater => println!("Bigger!"),
        Ordering::Equal =>{ 
            println!("You win!"); 
            break;}
        }
    }

}
