
mod classes;
mod assignments;

fn main() {
println!("Hello, world!");
  let mut x =5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // assignments::assignment_1::celsius_to_fahrenheit(100);
  // assignments::assignment_1::fahrenheit(212);
  // assignments::assignment_1::fibonacci_sequence(8);
  // assignments::assignment_1::lyrics_loop();

  class2();


}

fn class2() {
  // Ownership
  // let s1: String = String::from("Hello");
  // let s2: String = s1.clone();

  // println!("S1 print {}", s1);
  // println!("s2 print {}", s2);

  // let x1: String = String::from("Hello World!");
  // let y1: String = x1;
  // println!("The value of y1 is: {}", y1);

  // let len: () = calculate_length(&s1);
  
  // // Dereferencing
  // let num1: i32 = 7;
  // let num2: &i32 = &num1;
  // let num3: i32 = *num2;
  // println!("num1: {}, num2: {}, num3: {}", num1, num2, num3);

  // let hello_world: &str = "Hello World!";
  // let hello: &str = &hello_world[..5];
  // println!("{}", hello);
  
  // let hello_word:&str = "Hello Word!";
  // let word:&str = &hello_word[6..11];
  // println!("{}", word);


}

fn calculate_length(s: &String) {
  println!("Calculating length...{}", s);
  println!("The length of {} is: {}", s, s.len());

}

// Fibonacci number







