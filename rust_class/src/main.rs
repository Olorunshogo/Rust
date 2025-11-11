
use crate::assignments::assignment_1;
use crate::assignments::assignment_2;

mod assignments;

fn main() {

  assignment_1::fahrenheit_to_celsius(182);
  assignment_1::celsius_to_fahrenheit(100.0);
  assignment_1::fibonacci_number(12);
  assignment_1::fibonacci_sequence(12);
  // assignment_1::lyrics_loop_testing();

  assignment_2::run_coffee_shop_demo();

}