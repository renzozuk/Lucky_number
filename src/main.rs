use rand::Rng;

pub mod services;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    services::guess_number(secret_number, 1);
}

