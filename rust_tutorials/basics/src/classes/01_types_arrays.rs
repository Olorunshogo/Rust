// Data Types and Arrays
#[allow(dead_code)]
pub fn data_types_arrays() {
    println!("");
    println!("1. Data Types and Arrays");
    println!("");
    let numbers: [i32; 5] = [100; 5];
    println!("Number array: {:?}", numbers);

    println!("fruits Array");
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];

    println!("Fruits: {:?} ", fruits);
    println!("Fruits Array 1st element {:?} ", fruits[0]);
    println!("Fruiys Array 2nd element: {:?} ", fruits[1]);
    println!("Fruits Array 3rd element: {:?} ", fruits[2]);

    // Tuples contain heterogeneous collections of elements of fixed size
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1, 2, 3, 4, 5] - Contegeous playlist which means uninterrupted
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    let animals_slice: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animals_slice);

    let books_slice: &[&String] = &[
        &"Antelope".to_string(),
        &"Dog".to_string(),
        &"Cat".to_string(),
    ];
    println!("Other animals include: {:?}", books_slice);

    // String vs string slices
    let mut emmanuellas_name: String = String::from("Emmanuel");
    emmanuellas_name.push_str("la");
    println!("Emmanuella's name is: {}", emmanuellas_name);

    let owned_string: String = String::from("Hello, Owned String!");
    let string_slice1: &str = &owned_string;
    println!("String slice is: {}", string_slice1);
    let string_slice2: &str = &owned_string[0..5];
    println!("Range sliced value: {}", string_slice2);
}
