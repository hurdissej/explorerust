use std::io;
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number!");
    let secret_number = 9; //ToDo Replace with RAND package

    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
            
        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            }
        }
    }
}
