use rand::Rng;
use std::{cmp::Ordering, io::{self, Read}, vec};

fn main() {
    let s1 = String::from("hello world");
    let word = first_word_index(&s1);
    println!("{}", word);
}

// Goal: find end index of the first word 
fn first_word_index(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// Goal: return string of boolean
fn boolean_to_string(b: bool) -> String {
    match b {
        true => String::from("true"),
        false => String::from("false"),
    }
}


// Goal: Create fn that return avg
fn find_average(slice: &[f64]) -> f64 {
    let mut sum = 0.0;
    let length = slice.len() as f64;

    for i in slice.iter() {
        sum = sum + i;
    }

    sum/length
}

// Goal: Create a function with two arguments that will return an array of the first n multiples of x.
fn get_multiples(x: u32, n: u32) -> Vec<u32> {
    let mut vector = Vec::new();

    for i in 1..=n {
        vector.push(i as u32 * x);
    }

    vector
}

fn next_light(current: &str) -> String {
    let result = match current {
        "green" => "yellow".to_string(),
        "yellow" => "red".to_string(),
        "red" => "green".to_string(),
        _ => "please enter a valid color".to_string(),
    };
    println!("{}", result);
    return result;
}

fn fibonnaci(num: u32) -> u32 {
    // 0 1 1 2 3 5
    if num == 1 {
        return 0;
    }

    let mut first: u32 = 0;
    let mut second: u32 = 1;
    let mut result: u32 = 0;

    // for _number in 2..num {
        // result = first + second;
        // first = second;
        // second = result;
    // }
    let mut i = 0;

    while i < (num - 2) {
        result = first + second;
        first = second;
        second = result;
        i = i + 1;
    }

    println!("{}", result);
    return result;
}

fn control() {
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // loop {
    //     if number == 0 {
    //         break;
    //     };
    //     println!("number: {}", number);
    //     number -= 1;
    // }

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
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
