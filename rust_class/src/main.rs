
use crate::assignments::{ assignment_1, assignment_2 };
use crate::classes::bitcoin_dojo;
use crate::study::{ study_1, mod_factorial, study_2, study_3 };

mod assignments;
mod study;
mod classes;
mod tests;


fn main() {

  assignment_1::fahrenheit_to_celsius(182);
  assignment_1::celsius_to_fahrenheit(100.0);
  // assignment_1::fibonacci_number(12);
  // assignment_1::fibonacci_sequence(12);
  // assignment_1::lyrics_loop();

  // assignment_2::run_coffee_shop_demo();

  // Classes
  bitcoin_dojo::bitcoin_dojo_1();

  // Lessons
  // study_1();

  // mod_factorial::factorial(5);
  // mod_factorial::factorial_1(5);
  // mod_factorial::factorial_2(5);

  // println!("");
  // study_2();

  println!("");
  study_3();

  println!("This is bitcoin dojo class Field");

}


