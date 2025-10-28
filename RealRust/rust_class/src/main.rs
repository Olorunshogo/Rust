
fn main() {
println!("Hello, world!");
  let mut x =5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // bitcoin();

  rust_assignment_1();


}


// fn bitcoin() {
//   let a: u8 = 66;
//   let _b: f64 = 77.0;
//   let b: u8 = 66;
//   let _c: bool = false;

//   let _dddd: u8 = a + b;

//   let _e: char = 'Z';
//   let _f: &str = "I can write as much as I can here.";

//   // Tuple: Accomdate different data types
//   let tuple: (i32, &str, f64, u8) = (24, "Hey", 33.0, 22);
//   let _g: i32 = tuple.0;
//   let (_g, _h, _,_) = tuple;

//   // Arrays: Can only accomodate one data type
//   let _array1: [i32; 3] = [6, 68, 4875];
//   let _array2: [i32; 5] = [3;5];

//   // Functions
//   add(2, 8);

//   // Control Flow
//   let number = 3;
//   if number != 0 {
//     println!("The number is not equal to: {}", number);
//   }

//   let condition = true;
//   let number = if condition { 5 } else { 6 };

//   if number > 3 {
//     println!("The number is greater than: {}", number)
//   } else if number > 4 {
//     println!("The number is greater than: {}", number);
//   } else {
//     println!("I don't know what I'm doing but it should be less than: {}", number);
//   }

// //   loop {
// //     println!("I'll keep printing to infinity.")
// //   }

//     let mut counter: i32 = 0;
//     println!("Counting {counter}");
//     let result: i32 = loop {
//         counter += 1;

//         if counter == 10 {
//             break  counter * 2;
//         };
//     };

//     println!("The result of the loop is: {result}");

//     let mut count: i32 = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");

//     //  Conditional Loops with while
//     let mut number: i32 = 3;
//     while number !=0 {
//         println!("{number}!");
//         number -= 1;
//     }
//     println!("LIFT OFF!!!");

//     let for_array: [i32; 5] = [10, 20, 30, 40, 50];
//     for element in for_array {
//         println!("THe value is: {element}");
//     }

//     for _element in (1..4).rev() {
//         println!("{number}");
//     }
//     println!("Reverse LIFTOFF");





// }

// // println!("Hello Bitcoin {}", g);

// fn add(c: u8, d: u8) -> u32 {
//     let cal: u32 = (c + d) as u32;
//     println!("The value of {c} and {d} is ,{}", cal);
//     print!("Hey");
//     print!("Hello");
//     print!("--657747787497");
//     cal as u32
// }


fn rust_assignment_1() {
  fahrenheit_to_celsius(32);
  celsius_to_fahrenheit(90);
  // fibonacci_sequence(12);
  lyrics_loop();

}


// Convert temperatures between Fahrenheit and Celsius
fn fahrenheit_to_celsius(x: u32) -> u32 {
  let temperature = (x * (9/5)) + 32;
  println!("The value of {x} in °F is: {}", temperature);
  temperature
}

fn celsius_to_fahrenheit(x: u32) -> u32 {
  let temperature: u32 = (x - 32) / (9/5);
  println!("The value of {x} in °C is: {}", temperature);
  temperature
}

// Generate the nth Fibonacci number.
// fn fibonacci_sequence(n: u128) -> u128 {
//   let mut a: u128 = 0;
//   let mut b: u128 = 1;

//   for _ in 0..n {
//     let temp: u128 = a;
//     a = b;
//     b = temp + a;
//   }

//   a

//   println!("Fibonacci sequence for a is: {}", a);


// }


// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn lyrics_loop() {

  let christmas_lyrics: [&str; 12] = [
    "A partridge in a pear tree", "Two turtle doves and", 
    "Three french hens", "Four calling birds",
    "Five golden rings", "Six geese a-laying",
    "Seven swans a-swimming", "Eight maids a-milking",
    "On the ninth day of Christmas, my true love sent to me", "On the tenth day of Christmas, my true love sent to me",
    "On the eleventh day of Christmas, my true love sent to me", "Twelve drummers drumming"

  ];

  let positions: [u32; 7] = [1, 2, 3, 4, 5, 6, 7]; 
  for index in positions() {
    println!("Verse {}", christmas_lyrics[index]);
    println!("On the {} day of Christmas, my true love sent to me", christmas_lyrics[index]);
    
    for j in (0..=index).rev() {
      if j > 0 {
        println!("{}", christmas_lyrics[j]);      
      } else {
        // println!("");
      }
    }

    println!("{}", christmas_lyrics[0]);
    println!("DONE!!!");

  }
}



