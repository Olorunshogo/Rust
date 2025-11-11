
// TODO: Add some function with the name `call_me` without arguments or a return value.
fn call_me() -> i32 {
    let c: i32 = 3 + 4;
    println!("The sum of 3 and 4 is: {}", c);
    return c;
}


fn main() {
    call_me(); // Don't change this line
}
