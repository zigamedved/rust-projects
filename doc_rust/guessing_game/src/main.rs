use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // mutable

        io::stdin()
   	    .read_line(&mut guess)
	    .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // Parse returns a Result type and Result is an enum that has the variants Ok and Err.
	    Ok(num) => num,
	    Err(_) => continue, // Here we are matching all Err values, no matter what information they have inside them.
	};
    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too small!"),
	    Ordering::Greater => println!("Too big!"),
	    Ordering::Equal => {
	        println!("You win!");
		break;
	    }
        }
    }
}
