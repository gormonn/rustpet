use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Пожалуйста введи число!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok (num) => num,
            Err (_) => continue
        };

        println!("Ты загадал : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Секретное число: {secret_number}");
                break;
            },
        }
    }
}
