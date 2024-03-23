use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("GUESSING GAME");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess.");
        let mut guess= String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");   

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
