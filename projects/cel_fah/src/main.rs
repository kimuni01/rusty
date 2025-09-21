// a program that converts fahrenheit to celsius
// Formula: °C = (°F - 32) ÷ 1.8
use std::io;
// 1st attempt, failed
fn main() {
    let mut str1 = String::new();

    println!("Fahrenheit to Celsius : 0, Celsius to Fahrenheit : 1");

    io::stdin()
        .read_line(&mut str1)
        .expect("Failed to read line");

    let mut flt1:f64;

    if (str1 == "0") {
        print!("Enter Fahrenheit Temperature : ");
        io::stdin().read_line(&mut str1).expect("Failed to read line");
        flt1 = str1.trim().parse().expect("Given text was not a number");
        println!("Fahrenheit to Celsius : {(flt1 - 32) / 1.8}");
    }
    else if (str1 == "1") {
        print!("Enter Celsius Temperature : ");
        io::stdin().read_line(&mut str1).expect("Failed to read line");
        flt1 = str1.trim().parse().expect("Given text was not a number");
        println!("Celsius to Fahrenheit : {(flt1 * 1.8) + 32}");
    }
}
