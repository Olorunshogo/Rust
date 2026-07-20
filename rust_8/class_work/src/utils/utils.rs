// Util functions
#[allow(dead_code)]
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
      let result: f32 = (fahrenheit - 32.0) * (5.0 / 9.0);
      println!("The value of {} to celsius is: {}°C.", fahrenheit, result);
      return result;
  }
  
  #[allow(dead_code)]
  pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
      let result: f32 = (celsius - (9.0 / 5.0)) + 32.0;
      println!("The value of {} to celsius is: {}°C.", celsius, result);
      return  result;
  }