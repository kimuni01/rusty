// a program that converts fahrenheit to celsius
// Formula: °C = (°F - 32) ÷ 1.8
use std::io;
// 2nd attempt failed, str1.trim() makes it '&str' while 'String' is expected
// getting assists from Claude Sonnet 4
fn main() {
    let mut str1 = String::new();

    println!("Fahrenheit to Celsius : 0, Celsius to Fahrenheit : 1");

    io::stdin()
        .read_line(&mut str1)
        .expect("Failed to read line");

    let mut flt1:f64;
    
    str1 = str1.trim(); // if not done, it is "0\n" or "1\n" for expected inputs

    if (str1 == "0") {
        print!("Enter Fahrenheit Temperature : ");
        io::stdin().read_line(&mut str1).expect("Failed to read line");
        flt1 = str1.trim().parse().expect("Given text was not a number");
        println!("Fahrenheit to Celsius : {0}", ((flt1 - 32.0) / 1.8));
    } // I had to change '32' to '32.0' unless Rust compiler refuses 'float + integer'
    else if (str1 == "1") {
        print!("Enter Celsius Temperature : ");
        io::stdin().read_line(&mut str1).expect("Failed to read line");
        flt1 = str1.trim().parse().expect("Given text was not a number");
        println!("Celsius to Fahrenheit : {0}", ((flt1 * 1.8) + 32.0));
    }
}
