// Variables and mutability
#[allow(dead_code)]
pub fn define_x(x: &str) -> String {
    let result: String = x.to_string() + " world!";
    println!("{}", result);
    return result;
}

// Understanding Ownership
#[allow(dead_code)]
pub fn calculate_length(s: &String) -> usize {
    return s.len();
}

#[allow(dead_code)]
pub fn push_string(some_string: &str) -> String {
    let mut string = some_string.to_owned();
    string.push_str(", world!");
    return string;
}

#[allow(dead_code)]
pub fn push_mut_string(string: &mut String) {
    return string.push_str(", world!");
}

#[allow(dead_code)]
// pub fn dangling_pointers(dangling_arg: &str) -> String {
//     // let s: String = String::from("Dangling Hello");
//     let s: String = "Dangling Hello".to_string();
//     return &s;
// }
#[allow(dead_code)]
pub fn no_dangling_pointers(no_dangle: &str) -> String {
    let s: String = no_dangle.to_string();
    return s;
}

#[allow(dead_code)]
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Structs
#[allow(dead_code)]
pub fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

// Enums
#[allow(dead_code)]
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
