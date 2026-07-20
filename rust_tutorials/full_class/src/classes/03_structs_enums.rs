use crate::classes::structs_enums_03::{Coin::Quarter, UsState::Alabama};
#[allow(dead_code)]
use crate::utils::utils::{area, plus_one};
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
struct Point(i32, i32, i32);

#[allow(dead_code)]
struct AlwaysEqual;

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
// Using Structs to Structure Related Data
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
pub fn structs_enums_class() {
    println!("");
    println!("=== 3. Structs and Enums ===");
    println!("");

    let mut user_1: User = User {
        active: true,
        username: String::from("Someoneusername123@gmail.com"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user_1.email = String::from("anotheremail@example.com");

    #[allow(unused_variables)]
    let user_2: User = User {
        active: user_1.active,
        username: user_1.username,
        email: String::from("second@example.com"),
        sign_in_count: user_1.sign_in_count,
    };

    #[allow(unused_variables)]
    // Similar to user_2
    let user_3: User = build_user(
        String::from("thirdemail@example.com"),
        String::from("thirdusername"),
    );

    #[allow(unused_variables)]
    let black: Color = Color(0, 0, 0);

    #[allow(unused_variables)]
    let origin: Point = Point(0, 0, 0);

    #[allow(unused_variables)]
    let subject: AlwaysEqual = AlwaysEqual;

    structs_examples();

    println!();

    enum_class();

    println!();

    println!("Structs and Enums is successful");
}

// Structs with examples
#[allow(dead_code)]
pub fn dimensions_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[allow(dead_code)]
fn area_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

impl Rectangle {
    fn area_struct_method(&self) -> u32 {
        return self.width * self.height;
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }
}

#[allow(dead_code)]
fn structs_examples() {
    println!(
        "The area of the rectangle is {} square pizels.",
        area(30, 45)
    );

    let rect1: Rectangle = Rectangle {
        width: 45,
        height: 90,
    };
    println!(
        "The area of the rectangle, ret1 is {} square pizels.",
        area_struct(&rect1)
    );
    println!("rect1 is: {:?}.", rect1);
    println!("rect1 is: {:#?}.", rect1);
    println!(
        "The area of rect1 using struct method is {} square pizels.",
        rect1.area_struct_method()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is: {}.", rect1.width);
    }

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect2 is {:#?}", rect2);
    println!("rect3 is {:#?}", rect3);
}

#[allow(dead_code)]
#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

#[allow(dead_code)]
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn _value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}

impl UsState {
    pub fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

#[allow(dead_code)]
fn enum_class() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("The value of four is: {:?}", four);
    println!("The value of six is: {:?}", six);

    let home_1: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("The value of home is: {:#?}", home_1);
    // let home_2: IpAddr = IpAddr::V4(String::from("127.0.0.1"));

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("The value of loopback is: {:#?}", loopback);
    // let loopback = IpAddr::V6(String::from("::1"));

    let some_number: Option<i32> = Some(5);
    println!("The value of some_number is: {:?}.", some_number);
    let some_char: Option<char> = Some('e');
    println!("The value of some_char is: {:?}.", some_char);
    let absent_number: Option<i32> = None;
    println!("The value of absent_number is: {:?}.", absent_number);
    // The `match` Control Flow Construct

    let five: Option<i32> = Some(5);
    println!("The value of {:?} is {:?}.", five, five);
    let six: Option<i32> = plus_one(five);
    println!("The value of {:?} is {:?}.", six, six);
    let none: Option<i32> = plus_one(None);
    println!("The value of {:?} is {:?}.", none, none);

    // Dice roll
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    //
    describe_state_quarter(Quarter(Alabama));
}
