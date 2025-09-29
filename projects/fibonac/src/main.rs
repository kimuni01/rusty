// a program to generate 'n'th Fibonacchi sequence number
// 3rd attempt, I was having too much hard time fulfilling multiple goals at once,
// so I asked Gemini help.
// ideally I need to build up these by myself
// but I resorted to more passive learning this time.
use std::io;

fn main() {
    // --- Step 1: Get and Validate User Input ---
    println!("Generating the 'n'th Fibonacci number.");

    // We will loop until we get a valid number from the user.
    loop {
        println!("Please type a natural number : ");

        // 1a. Create a mutable String to hold the user's input.
        let mut input_string = String::new();

        // 1b. Read the line from the user.
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        // 1c. Parse the String into a number (u32).
        // .trim() removes whitespace and newlines.
        // .parse() attempts the conversion. It returns a `Result` type.
        let n: u32 = match input_string.trim().parse() {
            Ok(num) => num, // If parsing is succesfull, use the number.
            Err(_) => {
                // If it fails (e.g., user typed "test"), print an error and restart the loop.
                println!("Please enter a valid number!");
                continue;
            }
        };

        // --- Step 2: Calculate the Fibonacci Number ---
        if n == 0 {
            println!("The 0th Fibonacci number is 0.");
            break; // Exit the program
        }
        else if n == 1 {
            println!("The 1st Fibonacci number is 1.");
            break; // Exit the program
        }

        // Use more descriptive variable names.
        let mut a: u32 = 0;
        let mut b: u32 = 1;
        let mut counter = 2; // We start counting from the 2nd number.

        // The main calculation loop.
        while counter <= n {
            let next_fib = a + b;
            a = b;
            b = next_fib;
            counter += 1;
        }

        println!("The {}th Fibonacci number is {}.", n, b);
        break; // Exit the program after a successful calculation.
    }
}
