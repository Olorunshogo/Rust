use std::collections::HashMap;

// #[allow(dead_code)]
pub fn common_collections_class() {
    println!("");
    println!("=== COMMON COLLECTIONS: Vectors, UTF8, Hash Maps ===");
    println!("");

    // == Vectors, Vec<T>
    /*
     * Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in a memory.
     * Vectors can only store values of the same type. They are useful when you have a list of items such as the lines of text in a file or the prices of items in a shopping cart.
     */
    // Vec::new();
    println!("== Vectors ==");
    let mut vector_new: Vec<i32> = Vec::new();
    println!("The value of vector_new is: {:?}", vector_new);
    vector_new.push(1);
    vector_new.push(2);
    vector_new.push(3);
    vector_new.push(4);
    vector_new.push(5);
    vector_new.push(6);
    vector_new.push(7);
    vector_new.push(8);
    vector_new.push(9);
    println!("The value of vector_new is: {:?}", vector_new);

    // Starting a value with some values
    let mut _assigned_vec: Vec<i32> = Vec::new();
    let mut _assigned_vec: Vec<i32> = vec![1, 2, 3];
    println!("The value of assigned_vec is: {:?}", _assigned_vec);
    _assigned_vec.push(4);
    _assigned_vec.push(5);
    _assigned_vec.push(6);
    println!("The value of assigned_vec is: {:?}", _assigned_vec);
    let third_assigned_vec: &i32 = &_assigned_vec[2];
    println!("The value of the third value is: {}", third_assigned_vec);

    // Nerd snipping: Permanent link is: https://xkcd.com/356/ or HTTPS://XKCD.COM/356/

    let get_assigned_vec_third: Option<&i32> = _assigned_vec.get(2);
    match get_assigned_vec_third {
        Some(get_assigned_vec_third) => println!(
            "The third element for a GET method is: {}.",
            get_assigned_vec_third
        ),
        None => println!("there is no third element."),
    }

    // == UTF8
    println!("");
    println!("== UTF8 ==");

    let s_str: String = "whatever".to_string();
    println!("the value of s_str is: {}.", s_str);
    println!("the value of s_str is: {}.", s_str);
    println!("");

    let s_string: String = String::from("Whatever");
    println!("The value of s_string is: {}.", s_string);
    println!("The value of s_string is: {}.", s_string);
    println!("");

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2;
    println!("The value of s3 is: {}", s3);

    let mut full_bar: String = String::from("full ");
    println!("The value of full_bar is: {}.", full_bar);
    full_bar.push_str("bar!");
    println!("The value of full_bar is: {}.", full_bar);
    println!("");

    // Hash Maps
    /*
     * Hash Maps are used to store a mapping of keys to values of type V using a hashing function which determines how it places these keys and values into memory.
     * Many programming languages support this kind of data structure, but they often use a different name, such as hash map, object, hash table, dictionary, or associative array, just to name a few.
     * Hash maps are useful when you want to look un any data not by using an index, as you can with vectors, but by using a key that can be of any type.
     * For example, in a game, you could keep track of each team's score in a hash map in which each key is a team's name and the values are each team's score.
     * Given a team name, you can retireve its score
     */

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name: String = String::from("Blue");
    let get_scores = scores.get(&team_name).copied().unwrap_or(0);
    println!("The value of get scores is: {}", get_scores);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
