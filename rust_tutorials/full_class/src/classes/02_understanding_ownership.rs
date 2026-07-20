use crate::utils::utils::{
    calculate_length, first_word, no_dangling_pointers, push_mut_string, push_string,
};

#[allow(dead_code)]
pub fn understanding_ownership_class() {
    println!("");
    println!("=== 2. Understanding Ownership ===");
    println!("");

    let s1: String = String::from("Hello");

    let len = calculate_length(&s1);
    println!("The length of s {} is: {}.", s1, len);

    let new_string = push_string(&s1);
    let len_new_string = calculate_length(&new_string);

    println!("{}.", new_string);
    println!("The length of new_string is: {}.", len_new_string);

    let mut mutable_string: String = String::from("Mutable Hello");
    let len_mutable_string = calculate_length(&mutable_string);

    push_mut_string(&mut mutable_string);
    println!("{}.", mutable_string);
    println!("The length of mutable_string is: {}.", len_mutable_string);

    // References
    let mut s2: String = String::from("Mutable hello");
    {
        let r1: &mut String = &mut s2;
        println!("The value of r1 is: {}.", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s2;
    println!("The value of r2 is: {}.", r2);

    // let reference_to_nothing = dangling_pointers();
    let referrenced: String = no_dangling_pointers("no_dangle");
    println!("The value of referrenced is: {}.", referrenced);

    let word = first_word(&referrenced);
    println!("First space is at: {}", word);

    // String slice
    let slice_1: &str = &s2[0..2];
    println!("The value of the string slice is: {}", slice_1);

    let slice_2: &str = &s2[3..len];
    println!("The value of the string slice 2 is: {}", slice_2);

    let a_full: [u8; 6] = [1, 2, 3, 4, 5, 6];
    let a_full_slice: &[u8] = &a_full[1..3];
    assert_eq!(&[2, 3], a_full_slice);

    println!("Understanding ownership is successful");
}
