use std::io;
use rand::Rng;
use std::cmp::Ordering;
/* use std::env; */

fn main() {
/*     env::set_var("RUST_BACKTRACE", "1"); */

    println!("~~~ Guessing Game ~~~");
    
    loop{
    
        println!("Enter your guess (b/w 1 and 10):");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..=10);

        io::stdin() // can also use std::io::stdin if std::io is not imported at the start of the program
        .read_line(&mut guess)
        .expect("Failed to read line");

        // u32 will give error in runtime because u32 - u32 = u32 =/= negative

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Correct guess. You win.");
                break;
            }
            Ordering::Greater => println!("You are incorrect by {}.", guess - secret_number),
            Ordering::Less => println!("You are incorrect by {}.", guess - secret_number),
        }
    }
}
