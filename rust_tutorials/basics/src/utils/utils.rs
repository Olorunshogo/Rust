// Utility functions
#[allow(dead_code)]
pub fn print_string(x: &str) -> String {
    println!("{}", x);
    return x.to_string();
}

#[allow(dead_code)]
pub fn divide_two_values(numerator: f32, denominator: f32) -> f32 {
    let result = numerator / denominator;
    println!(
        "The result of dividing {} and {} is: {}.",
        numerator, denominator, result
    );
    return result;
}

#[allow(dead_code)]
pub fn divide_with_options(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[allow(dead_code)]
pub fn divide_with_result(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[allow(dead_code)]
pub fn feet_to_cm(height: f32) -> u32 {
    let height_cm: f32 = height * 30.48;
    println!("The value of {}ft in cm is: {}", height, height_cm);

    return height_cm.round() as u32;
}

#[allow(dead_code)]
pub fn cm_to_feet(height: u32) -> f32 {
    let height_ft: f32 = height as f32 / 30.48;
    println!("The value of {}cm in ft is: {}", height, height_ft);

    return height_ft.round() as f32;
}

#[allow(dead_code)]
pub fn name_information(name: &str, age: u32, _height: f32) {
    let height = feet_to_cm(_height);
    println!(
        "My name is {}, I am {} years old and my height is: {} cms",
        name, age, height
    );
}

#[allow(dead_code)]
pub fn calculate_product(price: u32, quantity: u32) -> u32 {
    let product: u32 = price * quantity;
    println!("The product of {} and {} is: {}.", price, quantity, product);

    return product;
}

#[allow(dead_code)]
pub fn calculate_bmi(weight: f64, height_cm: f64) -> f64 {
    let height_m: f64 = height_cm / 100.0;

    let bmi: f64 = weight / (height_m * height_m);
    println!("The BMI is: {:.2}.", bmi);
    return bmi;
}

#[allow(dead_code)]
pub fn calculate_string_length(s: &String) -> usize {
    let length: usize = s.len();
    println!("The length of {} is: {}.", s, length);
    return length;
}
