use rand::prelude::*;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        play();
        println!("\n🔁 Do you want to play again? (y/n): ");

        let mut answer = String::new();
        answer.clear();
        io::stdin().read_line(&mut answer).unwrap();

        if answer.to_string().trim().to_lowercase() == "y" {
            continue;
        }

        println!("\n👋 Goodbye! 😶‍🌫️");

        thread::sleep(Duration::from_secs(3));

        break;
    }
}

fn play() {
    let number: u32 = rand::rng().random_range(1..100);
    let mut guess = String::new();
    let mut luck: u8 = 7;

    println!("🎮 Welcome! \nEnter a number from 1 to 100 to start the game.\n");

    loop {
        if luck == 0 {
            println!("💀 The correct number was {}.", number);
            println!("❌ You lost!");
            break;
        }

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("⚠️ Failed to read line");

        if guess.is_empty() {
            println!("⚠️ Please enter a number!");
            continue;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number!");
                continue;
            }
        };

        if number == guess {
            println!("🎉 You guessed the number! 🏆");
            break;
        }

        if guess > number {
            println!("⬇️ It should be a smaller number.");
        }

        if guess < number {
            println!("⬆️ It should be a bigger number.");
        }

        luck -= 1;
        println!("❤️ Chances left: {}", luck);
    }
}
