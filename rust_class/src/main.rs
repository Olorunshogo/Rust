
use crate::assignments::assignment_1::{self, fibonacci_sequence, lyrics_loop, lyrics_loop_testing};


pub mod assignments;



fn main() {
    println!("Hello, Rust!");
    // let mut x =5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // let s = String::from("Hello world");
    // Absolute path
    crate::assignments::assignment_1::celsius_to_fahrenheit(100);
    

    // Relative paths
    assignment_1::celsius_to_fahrenheit(100);
    assignment_1::fahrenheit_to_celsius(234);
    assignment_1::fibonacci_number(12);

    fibonacci_sequence(12);
    lyrics_loop_testing();
    lyrics_loop();

    
    // class3();
    class4();
}



// 
// Bravoos Wallet Seed Phrase
// wreck artist address rural crop summer alone pattern thumb arrow rent tunnel

fn class4() {
    println!("This is the fourth class");
}




