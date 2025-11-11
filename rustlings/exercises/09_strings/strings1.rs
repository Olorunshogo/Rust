// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    let favorite_color = String::from("blue");
    return favorite_color;
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
