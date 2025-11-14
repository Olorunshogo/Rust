
use crate::assignments::{ assignment_1, assignment_2 };

use crate::study::{ study_1, study_2, factorial, factorial_1, factorial_2 };

mod assignments;
mod study;

fn main() {

  assignment_1::fahrenheit_to_celsius(182);
  assignment_1::celsius_to_fahrenheit(100.0);
  // assignment_1::fibonacci_number(12);
  // assignment_1::fibonacci_sequence(12);
  // assignment_1::lyrics_loop();

  assignment_2::run_coffee_shop_demo();

  // Lessons
  // study_1();
  study_2();

  let num = 5;
  println!("Factorial of {} is {}", num, factorial(num));
  println!("Factorial_1 of {} is {}", num, factorial_1(num));
  println!("Factorial_2 of {} is {}", num, factorial_2(num));


}


