// a program to generate 'n'th Fibonacchi sequence number
// 4th attempt, I asked Gemini for something and it produced
// improved version of the program.
// this works just fine, except it panics for input 48 or greater.
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

        // Allow the user to quit.
        if input_string.trim() == "quit" {
            break;
        }

        // Parse the input into a number.
        let n: u32 = match input_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number. Please try again.");
            continue; // Ask for input again // without continue it won't compile
            }
        };

        // --- The magic happens here! ---
        // Call our logic function and get the result.
        let result = fibonacci(n);

        println!("The {}th Fibonacci number is {}.", n, result);
        println!("----------------------------------------")
    }

    println!("Goodbye!");
}
