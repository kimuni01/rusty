fn main() {
    let width1 = 30; // v1
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        area(rect1) // v2 .v2
    ); // .v1
}

// fn area(width: u32, height: u32) -> u32 { // v1
//     width * height
// } // .v1

fn area(dimensions: (u32, u32)) -> u32 { // v2
    dimensions.0 * dimensions.1
} // .v2
