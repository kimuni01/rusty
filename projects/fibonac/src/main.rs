// a program to generate 'n'th Fibonacchi sequence number
// 2nd attempt, fixed 1st attempt with compiler errors
// made multiple blunders to get the final v2, only error being .read_line
// expected &mut String, found &mut u32 
use std::io;

fn main() {
    let mut result1 : u32 = 1; // this is for result
    let mut input1 : u32; // detached loop // v2
    loop {
        println!("Generating 'n'th Fibonacchi Sequence number");
        println!("Type a natural number : ");

        io::stdin()
            .read_line(&mut input1) // cannot use input1 in here yet // v2
            .expect("Not a valid input"); // how am I assured this would be an integer?

    } // missed semicolon at v1 // v2

    if input1 == 0 {
        println!("Answer is 0");
    }
    else if input1 == 1 && input1 == 2 {
        println!("Answer is 1");
    }
    else { // how do I check if input1 is a u32 variable? I want to use an else if.
        let mut cnt1 : u32 = 2; // this is for counting iterations inside while {}
        let mut iter1 : u32 = 1;
        let mut iter2 : u32 = 1; // this is redundant due to result1
        while (input1 < cnt1) {
            // let mut iter2 = iter1; // inference shall recognize iter2 is a u32 var.
            iter2 = iter1;
            iter1 = result1;
            result1 = iter2 + iter1;
            cnt1 += 1; // Rust has no ++/--, use += // v2
        }
        println!("Answer is {}", result1);
    }
    
}
