use std::io; // bringing input output library

fn main() { // the "entry point" of the program
    println!("Guess the number!"); // println! means it's a macro

    println!("Please input your guess."); // println! is not a function

    let mut guess = String::new(); // let = make a variable
    // mut = the variable is mutable, guess = the variable name
    io::stdin() // io::stdin() means stdin is an associated function of io
        .read_line(&mut guess)
        .expect("Failed to read line"); // until semicolon, these are
    // logically same line
    println!("You guessed : {guess}");
    
}
