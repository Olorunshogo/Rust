use crate::utils::utils::calculate_string_length;
// Ownership, Borrowing and References

// Ownership
#[allow(dead_code)]
pub fn ownership_class() {
    println!("");
    println!("3. Ownership, Borrowing and References");
    println!("");

    // 1.- Each value in Rust has a variable that's its owner.
    // 2.- There can be only one owner at a time.
    // 3.- When the owner goes out of scope, the value will be dropped

    let s1: String = String::from("RUST OWNERSHIP");
    calculate_string_length(&s1);
    let s2: String = s1;
    println!("{}", s2);
}

// 3. Third rule of owner. This value is out of scope
// println!("{}", s2);

// Borrowing and References
#[allow(dead_code)]
pub fn borrowing_class() {
    // Immutable References.
    // Mutable Reference.
    let mut _x: i32 = 5;

    println!("The value of _x is: {}.", _x);

    {
        let _r: &mut i32 = &mut _x;

        println!("The value of _r is: {}.", _r);

        *_r += 2;

        println!("The value of _r after adding 2 is: {}.", _r);
    } // _r goes out of scope here

    println!("Value of _x is: {}", _x);
}
