
use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn lessons_1() {
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

  let hello_rs = "Здравствуйте";
  let s = &hello[0..4];

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
  let score = scores.get(&team_name).copied().unwrap_or(0);
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







