use std::{
    cmp::Ordering,
    io::{stdin, BufRead},
};

pub fn guess_number(secret_number: u8, attempts: u8) {
    let mut buffer = String::new();

    loop {
        println!("Please, input your guess.");

        stdin()
            .lock()
            .read_line(&mut buffer)
            .expect("Failed to read your number.");

        buffer = buffer.trim().to_string();

        if let Ok(_) = buffer.parse::<u8>() {
            break;
        } else {
            buffer.clear();
        }
    }

    let guess: u8 = match buffer.trim().parse() {
        Ok(num) => num,
        Err(error) => panic!("Problem converting String to u8: {:?}", error),
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            guess_number(secret_number, attempts + 1);
        }
        Ordering::Greater => {
            println!("Too big!");
            guess_number(secret_number, attempts + 1);
        }
        Ordering::Equal => {
            println!("You win!");
            println!("Number of attempts: {attempts}");
        }
    }
}