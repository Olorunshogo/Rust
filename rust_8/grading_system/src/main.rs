mod assignments;
mod utils;

use crate::assignments::grading_system;

fn main() {
    println!("Hello, world!");

    calculate_grade(79);
    
    // Grading System
    println!("=== Grading System ===");
    grading_system::grading_system();

}

#[derive(Debug)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    F
}

fn calculate_grade(score: u8) -> Grade {
    let grade = match score {
        70..=100 => Grade::A,
        66..=69 => Grade::B,
        60..=65 => Grade::C,
        50..=59 => Grade::D,
        40..=49 => Grade::E,
        _ => Grade::F,
    };

    println!("Grade: {:?}", grade);
    return grade;
}
