// Shadowing
// Shadowing is very different from making a variable mutable
#[allow(dead_code)]
pub fn shadowing_class_6() {
    // x
    println!("");
    println!("6. Shadowing");
    println!("");

    let x: u8 = 5;
    println!("The value of x is: {}", x);
    let x: u8 = x + 22;
    println!("The value of the shadowed x is: {}", x);
    {
        let x: u8 = x * 2;
        println!("The value of the inner scope of x is: {}", x);
    }
    let x: u8 = x;
    println!("The value of down outer x is: {}", x);

    // Spaces
    let spaces: &str = "     ";
    let spaces: usize = spaces.len();

    println!("The length of spaces is: {}.", spaces);
}

// Control Flow
/*
   In any Programming language, controlling the flow of execution is based on two things, conditions and repeating actions.
   Together, they are called contro flow.
*/
#[allow(dead_code)]
pub fn control_flow_class() {
    println!("");
    println!("7. Control Flow");
    println!("");
    // If Else [ If expression ] [ Else expression ]
    let if_age: u8 = 1;
    let _is_adult: bool = if_age >= 19;
    if if_age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car!")
    }

    // Multiple conditions with else if
    let if_number: u16 = 6;
    if if_number % 300 == 0 {
        println!("Number is divisible by 4.");
    } else if if_number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else {
        println!("Number is divisible by 4, 3, or 2.");
    }

    let let_condition: bool = true;
    let let_number: u32 = if let_condition { 5 } else { 6 };
    println!("Number is: {}.", let_number);
}

/*
# Repetition with Loops:
# Doing Things Over and Over

Rust provides 3 types of loops:
    - Loops: Execute a block of code repeatedly until you explicitly tells it to stop
    - While:
    - For loop
*/

#[allow(dead_code)]
pub fn loops_while_for() {
    println!("");
    println!("=== LOOPS Loop ===");
    println!("");
    let mut loop_counter: u32 = 0;

    let loop_counter_result: u32 = loop {
        loop_counter += 2;
        println!("Loop_Counter is: {}", loop_counter);
        if loop_counter == 20 {
            break loop_counter * 2;
        }
    };

    println!(
        "The result of counter {} is: {}",
        loop_counter, loop_counter_result
    );

    // Loops Labels to Disambiguate Between Multiple Loops
    let mut count: u32 = 0;
    'counting_up: loop {
        println!("Count is: {}", count);
        let mut remaining: i32 = 10;

        loop {
            println!("Remaining == {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("The result of loop counter is: {}", count);

    // While loops
    println!("");
    println!("=== WHILE Loop ===");
    let mut while_num = 3;
    while while_num != 0 {
        while_num -= 1;
        println!("While number is: {}", while_num);
        break;
    }

    // Looping Through a Collection with for loop
    println!("");
    println!("=== FOR Loop ===");
    let for_number_array: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
    for number in for_number_array {
        println!("Number is: {}.", number);
    }
    println!();

    let for_letters_array: [&str; 2] = ["arch", "barrack"];
    for word in for_letters_array {
        println!("Word is: {}.", word);
    }
}
