use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el número");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // TODO: Count user attempts

    loop {
        println!("Por favor, escribe tu suposición (entre 1 y 100)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Es más grande"),
            Ordering::Greater => println!("Es más chico"),
            Ordering::Equal => {
                println!("Le atinaste crack!");
                break;
            }
        }
    }
}
