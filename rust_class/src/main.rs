
use crate::assignments::assignment_1;
use crate::assignments::assignment_2;
use crate::lessons::lessons_1;

mod assignments;
mod lessons;

fn main() {

  assignment_1::fahrenheit_to_celsius(182);
  assignment_1::celsius_to_fahrenheit(100.0);
  // assignment_1::fibonacci_number(12);
  // assignment_1::fibonacci_sequence(12);
  // assignment_1::lyrics_loop();

  // assignment_2::run_coffee_shop_demo();

  // Lessons
  lessons_1();

}