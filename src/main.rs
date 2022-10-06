use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    types();
}

fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn types() {
    // shadowing variables -> type change allowed
    let x = 5;
    println!("x is {}", x);
    let x = 10;
    println!("x is {}", x);
    let x = "test";
    println!("x is {}", x);

    let tup: (i32, f64, &str) = (500, 5.2, "test");
    let (x, y, z) = tup;
    println!("y is {}", tup.2);

    let arr = [1, 2, 3];
}
