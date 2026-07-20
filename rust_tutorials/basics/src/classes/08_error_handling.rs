use crate::utils::utils::{divide_two_values, divide_with_options, divide_with_result};

// = Approach 1: Option<T>
#[allow(dead_code)]
#[derive(Debug)]
enum Option<T> {
    // Define the generic Option type
    Some(T), // Represent a value
    None,    // Represents no value
}

// = Approach 2: Result<T,E>
#[allow(dead_code)]
#[derive(Debug)]
enum Result<T, E> {
    // Define the generic Result type
    Ok(T),  // Represents a value
    Err(E), // Represents an error
}

#[allow(dead_code)]
pub fn error_handling_class() {
    println!("");
    println!("=== ERROR HANDLING ===");
    println!("");

    // == Error Handling Techniques: 2 techniques
    /*
     * In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
     * This requirement makes your program more robust by ensuring that you'll discover errors and handle them before you've deployed your code to production!
     */

    divide_two_values(12.0, 24.0);

    // Example using approach 1

    let option_result_1 = divide_with_options(9.0, 3.0);
    match option_result_1 {
        Some(x) => println!("The value of option_result is: {}", x),
        None => println!("Error: Cannot divide by zero"),
    }

    let option_result_2 = divide_with_options(12.0, 0.0);
    match option_result_2 {
        Some(x) => println!("The value of option_result is: {}", x),
        None => println!("Error: Cannot divide by zero"),
    }

    // Example using approach 2
    let result_1 = divide_with_result(12.0, 0.0);
    match result_1 {
        Some(x) => println!("The value of option_result is: {}", x),
        None => println!("Error: Cannot divide by zero"),
    }
}
