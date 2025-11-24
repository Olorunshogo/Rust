
use crate::study::{ add, greeting };

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height < other.height
  }
}

struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    // if value < 1 || value > 100 {
    //   panic!("Guess value must be between 1 and 100, got {value}.");
    // }
    if value < 1 {
      panic!(
        "Guess value must be greater than or equal to 1, got {value}"
      );
    } else if value > 100 {
      panic!(
        "Guess value must be less than or equal to 100, got {value}"
      );
    }
    Guess { value }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_works() {
    let result: i32 = add(6, 2);
    assert_eq!(result, 8);
  }

  #[test]
  fn exploration() {
    let result: i32 = add(9, 7);
    assert_ne!(result, 20);
  }

  #[test]
  fn panic() {
    panic!("Make this test fail");
  }

  #[test]
  fn larger_can_hold_smaller() {
    let larger: Rectangle = Rectangle {
      width: 8, height: 7,
    };
    let smaller: Rectangle = Rectangle {
      width: 5, height: 2,
    };
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger: Rectangle = Rectangle {
      width: 8, height: 7,
    };
    let smaller: Rectangle = Rectangle {
      width: 5, height: 2,
    };
    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn greeting_contaims_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carol"),
      "Greetings did not contain name, value was `{result}`"
    );
  }

  #[test]
  #[should_panic(expected = "less than or equal to 100")]
  fn greater_than_100() {
    Guess::new(200);
  }

  #[test]
  fn it_should_work() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

}