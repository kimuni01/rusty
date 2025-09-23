// a program that converts fahrenheit to celsius
// Formula: °C = (°F - 32) ÷ 1.8
use std::io;
// 6th attempt, from loop & if to loop & match, the "idiomatic Rust"
// getting assists from Claude Sonnet 4
// Claude did heavy lifting for the 6th attempt
fn main() {
    println!("Fahrenheit to Celcius : 0, Celcius to Fahrenheit : 1"); // v6

    let choice = loop { // v6 // loop for assigning a value to "choice"
        let mut input = String::new(); // temporary variable input only for loop
        io::stdin() // reading the input
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() { // why does it not require .to_string() after .trim()?
            "0" => break "0",
            "1" => break "1",
            _ => println!("Please enter 0 or 1 : ") // the rest is _, but why no ; at the end?
        }
    }; // .v6


    match choice { // v6
        "0" => { // if "choice" is "0"
            println!("Enter Fahrenheit Temperature : ");
            let mut temp_input = String::new();
            io::stdin()
                .read_line(&mut temp_input)
                .expect("Failed to read line");
            let fahrenheit : f64 = 
            temp_input.trim().parse().expect("Given text was not a number");
            println!("Fahrenheit to Celsius : {}", (fahrenheit - 32.0) / 1.8);
        }
        "1" => {
            println!("Enter Celcius Temperature : ");
            let mut temp_input = String::new();
            io::stdin()
                .read_line(&mut temp_input)
                .expect("Failed to read line");
            let celsius : f64 = 
            temp_input.trim().parse().expect("Given text was not a number");
            println!("Celcius to Fahrenheit : {}", (celsius * 1.8) + 32.0);
        }
        _ => unreachable!() // what is this? not taught in the book yet.
    } // .v6
}
