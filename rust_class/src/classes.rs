
pub fn class1() {

    //   let a: u8 = 66;
    //   let _b: f64 = 77.0;
    //   let b: u8 = 66;
    //   let _c: bool = false;

    //   let _dddd: u8 = a + b;

    //   let _e: char = 'Z';
    //   let _f: &str = "I can write as much as I can here.";

    //   // Tuple: Accomdate different data types
    //   let tuple: (i32, &str, f64, u8) = (24, "Hey", 33.0, 22);
    //   let _g: i32 = tuple.0;
    //   let (_g, _h, _,_) = tuple;

    //   // Arrays: Can only accomodate one data type
    //   let _array1: [i32; 3] = [6, 68, 4875];
    //   let _array2: [i32; 5] = [3;5];

    //   // Functions
    //   add(2, 8);

    //   // Control Flow
    //   let number = 3;
    //   if number != 0 {
    //     println!("The number is not equal to: {}", number);
    //   }

    //   let condition = true;
    //   let number = if condition { 5 } else { 6 };

    //   if number > 3 {
    //     println!("The number is greater than: {}", number)
    //   } else if number > 4 {
    //     println!("The number is greater than: {}", number);
    //   } else {
    //     println!("I don't know what I'm doing but it should be less than: {}", number);
    //   }

    // //   loop {
    // //     println!("I'll keep printing to infinity.")
    // //   }

    //     let mut counter: i32 = 0;
    //     println!("Counting {counter}");
    //     let result: i32 = loop {
    //         counter += 1;

    //         if counter == 10 {
    //             break  counter * 2;
    //         };
    //     };

    //     println!("The result of the loop is: {result}");

    //     let mut count: i32 = 0;
    //     'counting_up: loop {
    //         println!("count = {count}");
    //         let mut remaining = 10;

    //         loop {
    //             println!("remaining = {remaining}");
    //             if remaining == 9 {
    //                 break;
    //             }
    //             if count == 2 {
    //                 break 'counting_up;
    //             }
    //             remaining -= 1;
    //         }

    //         count += 1;
    //     }
    //     println!("End count = {count}");
    //     //  Conditional Loops with while
    //     let mut number: i32 = 3;
    //     while number !=0 {
    //         println!("{number}!");
    //         number -= 1;
    //     }
    //     println!("LIFT OFF!!!");

    //     let for_array: [i32; 5] = [10, 20, 30, 40, 50];
    //     for element in for_array {
    //         println!("THe value is: {element}");
    //     }

    //     for _element in (1..4).rev() {
    //         println!("{number}");
    //     }
    //     println!("Reverse LIFTOFF");

    // }

    // // println!("Hello Bitcoin {}", g);

    // fn add(c: u8, d: u8) -> u32 {
    //     let cal: u32 = (c + d) as u32;
    //     println!("The value of {c} and {d} is ,{}", cal);
    //     print!("Hey");
    //     print!("Hello");
    //     print!("--657747787497");
    //     cal as u32
    // }
}


fn calculate_length(s: &String) {
    println!("Calculating length...{}", s);
    println!("The length of {} is: {}", s, s.len());
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub fn class2() {
    let mut user1 = User {
        active: true,
        username: String::from("someoneusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut user2 = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect1)
    );
    let scale: i32 = 2;
    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("rect2 is {rect2:?}");
    dbg!(&rect2);

    // Methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
}

pub fn class3() {
    student();

    let user1: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}



#[derive(Debug)]
struct Student<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: u8,
    gender: &'a str,
}

#[derive(Debug)]
enum Gender <'a> {
	MALE,
	FEMALE,
	OTHERS(&'a str)
}


impl<'v> Student<'v> {
    fn build_student<'T>(
        // Associate function
        first_name: &'T str,
        last_name: &'T str,
        age: u8,
        gender: &'T str,
    ) -> Student<'T> {
        Student {
            first_name,
            last_name,
            age,
            gender,
        }
    }

    fn name(&self) -> &str {
        // Method
        self.first_name
    }

    fn change_name(&mut self, new_name: &'v str) {
        self.first_name = new_name;
    }
}

fn student() {
    let student: Student<'_> = Student {
        first_name: "Uche",
        last_name: "Kabiru",
        age: 15,
        gender: "Female",
    };

    let s2: Student<'_> = Student {
        first_name: "Messi",
        age: 44,
        gender: student.gender,
        last_name: student.last_name,
    };

    println!("student data: {:#?}", student);
    println!("Small goat {} --1", s2.name());
	println!("Small goat {} --2", s2.name());
	println!("Small goat {} --3", s2.name());
    let s1: Student<'_> = Student {
        first_name: "",
        last_name: "",
        age: 57,
        gender: "Male",
    };
    dbg!("s1 data is: {:#?}", s1);
    println!("s1 data: {:#?}", s2);
    dbg!(&student);
    println!("Student data: {:#?}", student);
}

fn build_student<'T>(
    first_name: &'T str,
    last_name: &'T str,
    age: u8,
    gender: &'T str,
) -> Student<'T> {
    Student {
        first_name,
        last_name,
        age,
        gender,
    }
}



