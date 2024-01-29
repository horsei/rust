use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("guess the number");

    let secret_number= rand::thread_rng().gen_range(0..100);

    loop {
        println!("please input your guess");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32=match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };
        
        println!("you guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}","too big".red()),
            Ordering::Less => println!("{}","too small".red()),
            Ordering::Equal => {
                println!("{}","correct guess!".green());
                break;
            }
        }
    }

}
