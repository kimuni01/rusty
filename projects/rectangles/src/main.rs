fn main() {
    let width1 = 30; // v1
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    ); // .v1
}

fn area(width: u32, height: u32) -> u32 { // v1
    width * height
} // .v1