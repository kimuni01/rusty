// a program to generate 'n'th Fibonacchi sequence number
// 1st attempt, there are multiple vulnerabilities but I need to start.
use std::io;

fn main() {
    let mut result1 : u32 = 1; // this is for result
    let mut input1 : u32 = loop {
        println!("Generating 'n'th Fibonacchi Sequence number");
        println!("Type a natural number : ");

        io::stdin()
            .read_line(&mut input1)
            .expect("Not a valid input"); // how am I assured this would be an integer?

        if input1 == 0 {
            println!("Answer is 0");
            break;
        }
        else if input1 == 1 && input1 == 2 {
            println!("Answer is 1");
            break;
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
                cnt1++;
            }
            println!("Answer is {}", result1);
            break;
        }
    }
}
