
pub fn assignment_1() {
  // Convert temperatures between Fahrenheit and Celsius
  fn fahrenheit_to_celsius(x: u32) -> u32 {
    let celsius = (x * (9/5)) + 32;
    println!("The value of {x} in °F is: {}", celsius);
    return celsius;
  }

  fn celsius_to_fahrenheit(x: u32) -> u32 {
    let fahrenheit: u32 = (x - 32) / (9/5);
    println!("The value of {x} in °C is: {}", fahrenheit);
    return fahrenheit;
  }

  // Generate the nth Fibonacci number.
  fn fibonacci_sequence(n: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for i in 0..n {
      let temp: u128 = a;
      a = b;
      b = temp + a;
    }
    println!("Fibonacci sequence for b is: {}", b);

    return b;


  }


  // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
  fn lyrics_loop() {

    let lyrics_testing: [&str; 12] = [
      "A partridge in a pear tree", "Two turtle doves and", 
      "Three french hens", "Four calling birds",
      "Five golden rings", "Six geese a-laying",
      "Seven swans a-swimming", "Eight maids a-milking",
      "On the ninth day of Christmas, my true love sent to me", "On the tenth day of Christmas, my true love sent to me",
      "On the eleventh day of Christmas, my true love sent to me", "Twelve drummers drumming"

    ];

    let positions: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]; 
    for (index, day) in positions.iter().enumerate() {
      println!("Verse {}", positions[index]);
      println!("On the {} day of Christmas, my true love sent to me", lyrics_testing[index]);
      

      for j in (0..=index).rev() {
        if j > 0 {
          println!("{}", lyrics_testing[j]);
          // println!("{}", lyrics_testing[0]);
          // println!("");           
        } else {
          // println!("");
        }
      }

      println!("{}", lyrics_testing[0]);
      println!("");

    }
  }

}
