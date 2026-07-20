use crate::utils::utils;

#[allow(dead_code)]
pub fn var_mut_class() {
    println!("");
    println!("=== 1. Data Types and Arrays ===");
    println!("");

    // 1. A variable can only be used only if it has been initialized.
    // let x: i32;
    let a_1: i32 = 5;
    // let y: i32;
    let _y: i32;

    assert_eq!(a_1, 5);
    println!("a_1 is successful.");

    // 2. Use `mut` to mark a variable as mutable
    let mut x_mut: i32 = 3;
    x_mut = x_mut + 2;

    assert_eq!(x_mut, 5);
    println!("x_mut is successful.");

    // 3. Fix the error below with the least amount of modification
    let x_mod: i32 = 16;
    let y: i32 = 5;
    {
        println!("Tha value of x is {} and the value of y is: {}.", x_mod, y);
    }
    println!("Tha value of x is {} and the value of y is: {}", x_mod, y);

    // 4. Fix the error with the use of define_x
    utils::define_x("Shogo's");

    // 5. Shadowing
    // You can declare a new variable with the same name as a previous cariable, here we can say **the first one is shadowed by the second one**
    let shadow_x: i32 = 5;
    {
        let shadow_x: i32 = 12;
        assert_eq!(shadow_x, 12);
    }
    assert_eq!(shadow_x, 5);

    let shadow_x: i32 = 43;
    println!("The value of shadow_x in this scope is: {}", shadow_x);

    // Remove a line in the code to make it compile
    let mut remove_x: i32 = 24;
    println!("The value of shadow_x in this scope is: {}", remove_x);
    remove_x = 7;
    // Shadowing and re-binding
    let remove_x: i32 = remove_x;
    println!("The value of shadow_x in this scope is: {}", remove_x);

    // remove_x: i32 += 3;

    let remove_y: i32 = 56;
    println!("The value of shadow_x in this scope is: {}", remove_y);
    let remove_y: String = "I can also be bound to exit".to_string();
    println!("The value of shadow_x in this scope is: {}", remove_y);

    println!("Variables and mutability is successful");
}
