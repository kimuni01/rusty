// a program for printing lyrics of the song
// "The Twelve Days of Christmas", utilizing it's repetitiveness
// 5th attempt, I asked if 4th is perfect and got a few improvement
// so I didn't invest any thinking in 5th as Gemini explained to me.

fn main() {
    let lyr = [" patridge in a pear tree.",
    "Two turtle doves,", "Three French hens,", "Four calling birds,",
    "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,",
    "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,",
    "Eleven pipers piping,", "Twelve drummers drumming,"];

    let lyr2 = ["first", "second", "third", "fourth",
    "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh",
    "twelveth"]; // I also fixed the spelling and removed the comma from "third," for consistency :)

    println!("The Twelve Days of Christmas\n");

    for i in 0..11 {
        println!("On the {} day of Christmas,", lyr2[i]);
        println!("My true love sent to me");
        for j in (0..=11).rev() {
            // First, check if we're on the special "partridge" line.
            if j == 0 {
                // OK, it's the special line. NOW we check which day it is
                // to decide between "A" and "And a".
                if i == 0 {
                    println!("A{}", lyr[j]);
                }
                else {
                    println!("And a{}", lyr[j]);
                }
            }
            else {
                // If it's any  other line, just print it normally. No extra checks needed.
                println!("{}", lyr[j]);
            }
        }
        println!();
    }
}
