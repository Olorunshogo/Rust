use crate::utils::utils::{
    calculate_bmi, calculate_product, cm_to_feet, feet_to_cm, name_information, print_string,
};
// Functions class
#[allow(dead_code)]
pub fn functions_class() {
    println!("");
    println!("2. Functions");
    println!("");

    // Using an Util function: Put in `&str` to get a `String`
    let j: &str = "This is so cool man.";
    print_string(j);

    feet_to_cm(6.0);

    cm_to_feet(182);
    name_information("Henry", 400, 5.9);
    calculate_product(30, 40);
    calculate_bmi(76.0, 176.0);
}
