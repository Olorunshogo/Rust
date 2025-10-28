
fn main() {
    println!("Hello, world!");

    // Variable Bindings;
    let a = 1;
    println!("The value of a is: {}", a);

    // Patterns
    let (b, c) = (2, 3);
    println!("The value of x & y is: {} {}", b,c);

    // Type annotations
    let mut d: i32 = 5;
    println!("The value of x is: {}", d);

    // Mutability
    let mut e = 6;
    let e = 10;
    println!("THe value of e is: {}", e);

    print_sum(2, 7);

    // Expressions vs Statements
    // An expression returns value

    println!("This is amazing")
}

fn print_sum(x: i32, y: i32) {
    println!("Sum of x and y is: {}", x + y);
}
