// Main.rs

mod classes;
mod utils;

// use crate::classes::{functions_2, ownership_3, structs_4, types_arrays_1, shadowing_control_flow_6, enums_7, error_handling_8};
use crate::classes::common_collections_09;

// Constants
/* Multiline comment
A CONSTANT must not be declared with the `mut` keyboard, must be declared with its type and capital letter(s)
You can declare a constant globally with its type annotation here:
*/

#[allow(dead_code)]
const PI: f64 = 3.141592653;
#[allow(dead_code)]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// #[allow(warnings)]
fn main() {
    println!("Hello, World!");

    // Class 1: TYPES
    // types_arrays_1::data_types_arrays();

    // Class 2: FUNCTIONS
    // functions_2::functions_class();
    // functions_2::functions_class();

    // Class 3: OWNERSHIP
    // ownership_3::ownership_class();
    // ownership_3::borrowing_class();

    // Class 4: Structs
    // structs_4::structs_class();
    // structs_4::variables_immutability();
    // structs_4::tuples();

    // Class 5: CONSTANTS
    // println!("");
    // println!("5. CONSTANTS");
    // println!("");
    // println!("The value of I is: {}", PI);
    // println!(
    //     "The value of 3 hours in seconds is is: {}s",
    //     THREE_HOURS_IN_SECONDS
    // );

    // Class 6: Shadowing & Control Flow
    // shadowing_control_flow_6::shadowing_class_6();
    // shadowing_control_flow_6::control_flow_class();
    // shadowing_control_flow_6::loops_while_for();

    // Class 7: Enums
    // enums_7::enums_class();

    // Class 8: Error handling
    // error_handling_8::error_handling_class();

    // Class 9: Common Collections
    common_collections_09::common_collections_class();
}
