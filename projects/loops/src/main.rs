fn main() {
    // loop { // has to be quit with Ctrl + C in terminal // v1
        // println!("again!");
    // } // .v1

    // let mut counter = 0; // v2

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2; // this is how to return a value of a loop
    //     }
    // };

    // println!("The result is {result}"); // .v2


    // let mut count = 0; // v3
    // 'counting_up : loop { // loop with a label, for a 'break'
    //     println!("count = {count}");
    //     let mut remaining = 10;

        // loop {
        //     println!("remaining = {remaining}");
        //     if remaining == 9 {
        //         break;
        //     }
        //     if count == 2 {
        //         break 'counting_up; // escapes the outer 'loop'
        //     }
        //     remaining -= 1;
        // }

    //     count += 1;
    // }
    // println!("End count = {count}"); // .v3


    // let mut number = 3; // v4
    // while number != 0 { // shortens the code made of loop(s)
    //     println!("{number}"); // no break, no if, no else

    //     number -= 1;
    // }
    // println!("LIFTOFF!!!"); // .v4


    let a = [10, 20, 30, 40, 50]; // v5
    let mut index = 0;
    while index < 5 {
        println!("the value is : {}", a[index]); // new way of println introduced
        index += 1;
    } // .v5
}
