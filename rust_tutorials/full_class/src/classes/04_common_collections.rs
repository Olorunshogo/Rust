use std::collections::HashMap;
//

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Text(String),
    Float(f64),
}

// === Common Collections
#[allow(dead_code)]
pub fn common_collections_class() {
    println!("");
    println!("=== 4. Common Collections ===");
    println!("");

    // 8.1 Storing List of Values with Vectors
    storing_list_of_values_with_vectors();

    println!("");

    storing_encoded_text_with_string();

    println!("");

    storing_keys_with_associated_values();

    println!("");

    println!("Common collections is successful");
}

// 8.1 - Storing List of Values with Vectors
pub fn storing_list_of_values_with_vectors() {
    /*Create a new, empty vector
     * Vectors can only store calues that of the same type.
     */
    let _v: Vec<i32> = Vec::new();

    let mut vector_1: Vec<i32> = vec![1, 2, 3, 4, 5];
    vector_1.push(6);
    vector_1.push(7);
    vector_1.push(8);
    vector_1.push(9);

    let last = vector_1.push_mut(112);
    *last += 2;

    for v in vector_1 {
        println!("V1: {}, ", v);
    }

    let mut vector_2: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third_1: &i32 = &vector_2[2];
    println!("The third element is: {}", third_1);

    let third_2: Option<&i32> = vector_2.get(2);
    match third_2 {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist_1 = &vector_2[100];
    // println!("There is no value at this index: {}.", does_not_exist_1);
    let does_not_exist_2 = &vector_2.get(100);
    println!("There is no value at this index: {:?}.", does_not_exist_2);

    for i in &mut vector_2 {
        *i += 50;
        println!("Index + 50 is: {i}");
    }

    // Using an Enum to Store Multiple Types
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Spreadsheet cell is: {:?}", row);

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int is: {}", i),
            SpreadsheetCell::Text(s) => println!("Text is: {}", s),
            SpreadsheetCell::Float(f) => println!("Float is: {}", f),
        }
    }

    // Dropping a Vector Drops Its Elements
    // Like any other struct, a vector is freed when it goes out of scope, as annotated in Listing 8-10.

    // {
    //     let v: Vec<i32> = vec![1, 2, 3, 4];

    //     // do stuff with v
    // } // <- v goes out of scope and is freed here
    /*
     * Listing 8-10: Showing where the vector and its elements are dropped
     * When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
     */
}

// 8.2 - Storing UTF-8 Encoded Text with Strings
pub fn storing_encoded_text_with_string() {
    /*Create a new, empty vector
     * Vectors can only store calues that of the same type.
     */
    let mut string_1: String = String::new();

    let data = "Initial contents".to_string();
    string_1.push_str(&data);

    println!("String 1 is: {}", string_1);

    let hello_01: String = String::from("السلام عليكم");
    println!("Hello 1 is: {}", hello_01);
    println!("The length of {hello_01} is: {}.", hello_01.len());

    let hello_02: String = String::from("Dobrý den");
    println!("Hello 2 is: {}", hello_02);
    println!("The length of {hello_02} is: {}.", hello_02.len());

    let hello_03: String = String::from("Hello");
    println!("Hello 3 is: {}", hello_03);
    println!("The length of {hello_03} is: {}.", hello_03.len());

    let hello_04: String = String::from("שלום");
    println!("Hello 4 is: {}", hello_04);
    println!("The length of {hello_04} is: {}.", hello_04.len());

    let hello_05: String = String::from("नमस्ते");
    println!("Hello 5 is: {}", hello_05);
    println!("The length of {hello_05} is: {}.", hello_05.len());

    let hello_06: String = String::from("こんにちは");
    println!("Hello 6 is: {}", hello_06);
    println!("The length of {hello_06} is: {}.", hello_06.len());

    let hello_07: String = String::from("안녕하세요");
    println!("Hello 7 is: {}", hello_07);
    println!("The length of {hello_07} is: {}.", hello_07.len());

    let hello_08: String = String::from("你好");
    println!("Hello 8 is: {}", hello_08);
    println!("The length of {hello_08} is: {}.", hello_08.len());

    let hello_09: String = String::from("Olá");
    println!("Hello 9 is: {}", hello_09);
    println!("The length of {hello_09} is: {}.", hello_09.len());

    let hello_10: String = String::from("Здравствуйте");
    println!("Hello 10 is: {}", hello_10);
    println!("The length of {hello_10} is: {}.", hello_10.len());

    let hello_11: String = String::from("Hola");
    println!("Hello 11 is: {}", hello_11);
    println!("The length of {hello_11} is: {}.", hello_11.len());

    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");

    println!("Tic-tac-toe is: {s1}-{s2}-{s3}");

    let format_s = format!("{s1}-{s2}-{s3}");
    println!("Format_s is: {}", format_s);

    // A `String` is a wrapper over a `Vec<u8>`. Let's look at some of our properly encoded UTF-8 example strings.
    println!("");

    let sliced_hello_01 = &hello_01[0..10];
    println!("Sliced {hello_01} run is: {sliced_hello_01}");

    //
    let mut hello_02_bytes: Vec<u8> = Vec::new();

    for b in hello_02.bytes() {
        hello_02_bytes.push(b);
    }
    println!("Hello 2 bytes is: {:?}", hello_02_bytes);
}

// 8.3 - Storing Keys with Associated Values in Hash Maps
pub fn storing_keys_with_associated_values() {
    // Storing Keys with Associated Values in Hash Maps
    /* The last of our common collections is the hash map. The type `HashMap<K, V>` stores a mapping of keys of type `K`to values of type `V` using a hashing function, which determines how it places these keys and values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.
     * Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type. For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score. Given a team name, you can retrieve its score.
     */

    // Creating a New Hash Map
    let mut scores: HashMap<String, u8> = HashMap::new();

    scores.insert(String::from("Chelsea"), 24);
    scores.insert(String::from("Chelsea"), 42);
    scores.insert(String::from("Manchester"), 23);
    scores.entry(String::from("Liverpool")).or_insert(50);
    scores.entry(String::from("Manchester")).or_insert(32);

    // Accessing Values in a Hash Map
    let team_name: String = String::from("Manchester");
    let score: u8 = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score is: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Managing Ownership in Hash Maps
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.

    let colour_key = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(colour_key, field_value);
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Updating a Hash Map
}
