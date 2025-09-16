fn main() {
    // let mut x = 5; // v1 to v2
    // println!("The value of x is : {x}");
    // x = 6;
    // println!("The value of x is : {x}"); // .v1

    
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant variable example

    let x = 5; // v3
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is : {x}"); // .v3
}
