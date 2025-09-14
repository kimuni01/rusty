use std::io; // bringing input output library
use std::cmp::Ordering; // Ordering is an enum type, has 3 variants Less, Greater, Equal. // v3
use rand::Rng; // library for random numbers. trait that defined multiple methods. // v2

fn main() { // the "entry point" of the program
    println!("Guess the number!"); // println! means it's a macro

    let secret_number = rand::thread_rng().gen_range(1..=100); // v2

    println!("The secret number is : {secret_number}"); // v2

    loop { // v5
        println!("Please input your guess."); // println! is not a function

        let mut guess = String::new(); // let = make a variable
        // mut = the variable is mutable, guess = the variable name
        io::stdin() // io::stdin() means stdin is an associated function of io
            .read_line(&mut guess)
            .expect("Failed to read line"); // until semicolon, these are
        // logically same line

        let guess : u32 = guess.trim().parse().expect("Please type a number!"); // v4

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) { // cmp method, shall compare 2 values ... // v3
            Ordering::Less => println!("Too small!"), // match is consisted of arms.
            Ordering::Greater => println!("Too big!"), // an arm is made of a pattern
            // and code(s) to run if an arm's pattern is met.
            Ordering::Equal => {
                println!("You win!"); // v6
                break; // v6
            }
        }
    } // v5
    
}
