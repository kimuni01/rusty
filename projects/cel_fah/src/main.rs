// a program that converts fahrenheit to celsius
// Formula: °C = (°F - 32) ÷ 1.8
use std::io;
// 3rd attempt, working but print! is faulty
// getting assists from Claude Sonnet 4
fn main() {
    let mut str1 = String::new();

    println!("Fahrenheit to Celsius : 0, Celsius to Fahrenheit : 1");

    io::stdin()
        .read_line(&mut str1)
        .expect("Failed to read line");

    let mut flt1: f64; // compiler suggests the mut is not necessary, but anyways
    
    str1 = str1.trim().to_string(); // if not done, it is "0\n" or "1\n" for expected inputs

    if str1 == "0" { // I prefer () for 'if' but this time.
        print!("Enter Fahrenheit Temperature : ");
        let mut temp_input = String::new(); // use new variable for temperature input // v3
        io::stdin().read_line(&mut temp_input).expect("Failed to read line");
        flt1 = temp_input.trim().parse().expect("Given text was not a number"); // .v3

        // io::stdin().read_line(&mut str1).expect("Failed to read line"); // v2
        // flt1 = str1.trim().parse().expect("Given text was not a number"); // .v2

        println!("Fahrenheit to Celsius : {}", ((flt1 - 32.0) / 1.8)); // {0} not necessary
    } // I had to change '32' to '32.0' unless Rust compiler refuses 'float + integer'
    else if (str1 == "1") {
        print!("Enter Celsius Temperature : ");
        let mut temp_input = String::new(); // v3
        io::stdin().read_line(&mut temp_input).expect("Failed to read line");
        flt1 = temp_input.trim().parse().expect("Given text was not a number"); // .v3

        // io::stdin().read_line(&mut str1).expect("Failed to read line"); // v2
        // flt1 = str1.trim().parse().expect("Given text was not a number"); // .v2

        println!("Celsius to Fahrenheit : {0}", ((flt1 * 1.8) + 32.0));
    }
}
