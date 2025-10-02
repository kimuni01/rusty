// a program to generate 'n'th Fibonacchi sequence number
// 5th attempt, probably the last version with u32. got help from Gemini
// modified the input handling part to prevent panic for input > 47
// 2..=n is from 2 to n for integers
use std::io;

// (Paste the  fibonacci function from above right here)
fn fibonacci(n : u32) -> u32 {
    // ... logic from Step 1 ...
    if n == 0 { return 0; }
    else if n == 1 { return 1; }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n { // 2..n is from 2 to n - 1 for integers
        let next_fib = a + b;
        a = b;
        b = next_fib;
    }
    b
}

fn main() {
    println!("Generating the 'n'th Fibonacci number.");

    loop {
        println!("Please type a natural number (or 'quit' to exit):");
        let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        if input_string.trim() == "quit" {
            break;
        }

        let n : u32 = match input_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number. Please try again.");
                continue;
            }
        };

        // --- NEW: Add a check to prevent overflow ---
        // The 47th Fibonacci number is the largest that fits in a u32.
        if n >= 48 {
            println!("⚠️ Warning: An input of 48 or greater will cause an overflow.");
            println!("Please enter a number between 0 and 47.");
            println!("--------------------------------------------------");
            continue; // Skip the calculation and ask for input again
        }

        // --- This part now only runs for valid numbers ---
        let result = fibonacci(n);
        println!("The {}th Fibonacci number is {}.", n, result);
        println!("--------------------------------------------------");
    }

    println!("Goodbye!");
}
