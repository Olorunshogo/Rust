use std::fs::File;
use std::io::ErrorKind;
#[allow(unused_imports)]
use std::io::{self, Read};
use std::net::IpAddr;

#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[allow(dead_code)]
enum Option<T, E> {
    Some(T),
    None(E),
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

// === Error Handling
#[allow(dead_code)]
pub fn error_handling_class() {
    println!("");
    println!("=== 5. Error Handling ===");
    println!("");

    // 9.1 - Unrecoverable Errors with panic!
    /*
     * Sometimes bad things happen in our code, and there's nothing you can do about it. In these cases, Rust has the `panic!` macro.
     */

    // panic!("Crash and burn");

    // let error_1 = vec![1, 2, 3, 4, 5];
    // error_1[99];

    let greeting_file_result = File::open("./hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(_e) => panic!("Problem creating the file: {error:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };
    println!("{:?}", greeting_file);

    // Unwrap here will return the value inside the `OK`
    let greeting_file_unwrap = File::open("./hello.txt").unwrap();
    println!("Greeting file unwrap is: {:?}", greeting_file_unwrap);

    /*Similarly, `expect` method lets us also choose the `panic!` error message.
     * Using `expect` insead of `unwrap` and and providing good error messages can convey your intent and make tracking down the source of a panic easier.
     * The syntax of expect looks like this:
     */
    let greeting_file_expect =
        File::open("hello.txt").expect("hello.txt should be included in this project");
        println!("Greeting file expect is: {:?}", greeting_file_expect);

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("Home is: {}", home);

        let secret_number = 42;

        println!("Guess the number!");
        
        loop {
            println!("Please input your guess.");
        
            let mut guess = String::new();
        
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
        
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        
            if guess < 1 || guess > 100 {
                println!("The secret number must be between 1 and 100.");
                continue;
            }
        
            println!("You guessed: {guess}");
        
            match guess.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("Too small!"),
                std::cmp::Ordering::Greater => println!("Too big!"),
                std::cmp::Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }

    println!("");

    println!("Error Handling is successful");
}
