
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
  Int(u32),
  Float(f64),
  Text(String),
}

pub fn study_1() {
  println!("This is study 1");

  let mut v: Vec<i32> = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  let third: &i32 = &v[2];
  println!("The third element is: {}", third);

  let third: Option<&i32> = v.get(2);
  match third {
    Some (third) => println!("The third element is: {}", third),
    None => println!("There is no third element"),
  }

  let v = vec![100, 250, 323, 467, 598];
  let first =  &v[0];

  println!("The first element is: {}", first);
  
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text(String::from("Blue")),
  ];

  println!("The value of row is: {:?}", row);

  {
    let scoped_v = vec![1, 2, 3, 4];
    println!("The value of scoped v is: {:?}", scoped_v);
  }

  // String
  let s = String::new();
  println!("The value of s is: {}", s);
  println!("The value of s is: {s}");

  let data = "Initial contents";
  let s = data.to_string();
  println!("The value of s is: {s}");

  // The method also works on a literal directly:
  let s = "Initial contents".to_string();
  println!("The value of s is: {s}");

  let s = String::from("Initial contents");
  println!("The value of s is: {s}");

  // let hello = String::from("السلام عليكم");
  // let hello = String::from("Dobrý den");
  // let hello = String::from("Hello");
  // let hello = String::from("שלום");
  // let hello = String::from("नमस्ते");
  // let hello = String::from("こんにちは");
  // let hello = String::from("안녕하세요");
  // let hello = String::from("你好");
  // let hello = String::from("Olá");
  // let hello = String::from("Здравствуйте");
  let hello = String::from("Hola");

  println!("Hello in the last language is: {}", hello);

  let mut s1 = String::from("Foo");
  let s2 = "Bar";
  s1.push_str(s2);  
  println!("The value of s2 is: {}", s2);
  println!("The value of s1 is: {}", s1);

  let (s1, s2) = (String::from("Hello, "), String::from("world!"));
  println!("The value of s1 is: {}, and s2 is: {}.", s1, s2);

  let s3 = s1 + &s2;
  println!("The value of s3 is: {}", s3);

  let (s1, s2, s3) = (String::from("tic"), String::from("tac"), String::from("toe"));
  let s = s1 + "-" + &s2 + "-" + &s3;
  println!("S is: {}", s);

  for c in "Зд".chars() {
    println!("{c}");
  }

  // for b in "Здравствуйте".chars() {
  //   println!("{b}");
  // }

  // for b in "Здравствуйте".bytes() {
  //   println!("{b}");
  // }

  // Hashmap
  // Creating a hashmap and inserting some keys and values
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let _score: i32 = scores.get(&team_name).copied().unwrap_or(0);
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);


  for (key, value) in &scores {
    println!("{key}: {value};");
  }

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);

  println!("{scores:?}");

  let text = "Hello wonderful world!";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{map:?}");

}


enum Result<T, E> {
  Ok(T),
  Err(E),
}

pub mod mod_factorial {
  pub fn factorial(n: u64) -> u64 {
    let result: u64 = if n == 0 {
      1
    } else {
      n * factorial_1(n - 1)
    };

    println!("Factorial of {} is: {}.", n, result);
    return result;
  }

  pub fn factorial_1(n: u64) -> u64 {
    let result: u64 = (1..=n).product();
    println!("Factorial_1 of {} is: {}.", n, result);
    return result;
  }

  pub fn factorial_2(n: u64) -> u64 {
    let result: u64 = (1..=n)
      .map(|x: u64| x) // mapping each number to itself (could transform here if needed)
      .reduce(|acc: u64, x: u64| acc * x)  // multiply all together
      .unwrap_or(1); // handle the empty case (like factorial(0))
      println!("Factorial_2 of {} is: {}.", n, result);
      return result;
  }

}

use std::ffi::c_float;
use std::fmt::{format, Display};
use std::io::Split;
use std::net::IpAddr;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Point<T, U> {
  x: T,
  y: U,
}

struct PointXY<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> PointXY<X1, Y1> {
  fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
    PointXY { 
      x: self.x, 
      y: other.y,
    }
  }
}

trait Summary {
  fn summarize_author(&self) -> String;

  // fn summarize(&self) -> String {
  //   String::from("(Read more...)")
  // }

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}  

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("@{}", self.headline)
  }

  fn summarize(&self) -> String {
    format!("{}, by {} ({})",
      self.headline, self.author, self.location
    )
  }
}

pub struct SocialPost {
  username: String,
  content: String,
  reply: bool,
  repost: bool,
}

impl Summary for SocialPost {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }

  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}



trait MakeNoise {
  fn make_noise(&self);
}
struct Lion;
struct Parrot;

impl MakeNoise for Lion {
  fn make_noise(&self) {
    println!("ROAR");
  }
}

impl MakeNoise for Parrot {
  fn make_noise(&self) {
    println!("Pretty bird!");
  }
}

fn feeding_time(animal: &impl MakeNoise) {
  println!("Feeding time!");
  animal.make_noise();
}

fn longest_with_announcement<'a, T>(
  x: &'a str, 
  y: &'a str,
  ann: T,
) -> &'a str
  where T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn largest_i32(list: &[i32]) -> &i32 {
  let mut largest: &i32 = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  return largest;
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }

  return largest;
}

fn largest_char(list: &[char]) -> &char {
  let mut largest: &char = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  return largest;
}



pub fn study_2() {
  println!("");
  println!("This is study 2");

  // panic!("Crash and burn");
  // let v = vec![1, 2, 3];
  // let v1 = v[99];
  // println!("{:?}", v);
  // println!("{:?}", v1);

  // let greeting_file_result = File::open("hello.txt");
  // format!({:?}, greeting_file_result);
  let _home: IpAddr = "127.0.0.1"
      .parse()
      .expect("Hardcoded Ip address should be valid");

  // Generic Types, Traits, and Lifetimes
  // Generic Data Types
  let number_list: Vec<i32> = vec![34, 50, 25, 100, 45];
  let result: &i32 = largest_i32(&number_list);
  println!("The largest number is: {}", result);

  let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
  let result: &char = largest_char(&char_list);
  println!("The largest char is: {}", result);

  let number_list: Vec<i32> = vec![34, 50, 25, 100, 45];
  let result: &i32 = largest(&number_list);
  println!("The largest number is: {}", result);

  let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
  let result: &char = largest(&char_list);
  println!("The largest char is: {}", result);

  let integer: Point<i32, i32> = Point { x: 5, y: 10 };
  println!("Integer: {:#?}", integer);

  let float: Point<f64, f64> = Point { x: 1.0, y: 4.0 };  
  println!("Float: {:#?}", float);

  let integer_and_float: Point<i32, f64> = Point { x: 5, y: 4.0 };
  println!("Float: {:#?}", integer_and_float);

  let p1: PointXY<i32, f64> = PointXY { x: 5, y: 10.4 };
  let p2: PointXY<&str, char> = PointXY{ x: "Hello", y: 'c' };
  let p3: PointXY<i32, char> = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

  let post: SocialPost = SocialPost {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    repost: false,
  };

  println!("1 new post: {}", post.summarize());

  let article: NewsArticle = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
      hockey team in the NHL.",
    ),
  };

  println!("New article available! {}", article.summarize());

  // feeding_time(parrot);

  let x: i32 = 5;
  let r: &i32 = &x;

  println!("The value of r is: {}", r);

  let string1: String = String::from("abcd");
  let string2: &str = "xyz";

  let string_result = longest_with_announcement(string1.as_str(), string2, "Comparing strings");
  println!("The longest string is: {}", string_result);

  let novel: String = String::from("Call me Ishmael. Some years ago...");
  let first_sentence: &str = novel.split('.').next().unwrap();
  let _i: ImportantExcerpt<'_> = ImportantExcerpt {
    part: first_sentence,
  };

}


fn add_two(num: i32) -> i32 {
  let result: i32 = num + 2;
  println!("The sum of {} and 2 is: {}.", num, result);
  return result;
}

fn add_ten(num: i32) -> i32 {
  let result: i32 = num + 10;
  println!("The sum of {} and 10 is: {}.", num, result);
  return result;
}

fn minus_fifty(num: i32) -> i32 {
  let result: i32 = num - 50;
  println!("The difference of {} and 50 is: {}.", num, result);
  return result;
}

pub fn add(left: i32, right: i32) -> i32 {
  let sum: i32 = left + right;
  println!("The sum of {} and {} is: {}.", left, right, sum);
  return sum;
}

pub fn greeting(name: &str) -> String {
  let full_name: String = format!("Hello {}", name);
  return full_name;
}

pub fn study_3() {
  println!("");
  println!("11. Writing to Write Tests");
  println!("11.1 - Controlling How Tests Are Run");
  println!("");

  add_two(7);
  add_ten(50);
  minus_fifty(30);
  add(30, -30);

  println!("");
  greeting("Olorunshogo");
}



