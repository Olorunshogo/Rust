// Structs
// STRUCT: A data structure that allows you to group multiple fields together under one name.

#[allow(dead_code)]
pub fn structs_class() {
    println!("");
    println!("4. Structs");
    println!("");
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow
    account.withdraw(50.550);

    account.check_balance();

    println!("");

    let mut user1: User = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("fromusername@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("User1 email: {}", user1.email);
    println!("User1 username: {}", user1.username);
    println!("User1 active: {}", user1.active);
    println!("User1 sign_in_count: {}", user1.sign_in_count);

    let built_user = build_user(
        "buildemail@gmail.com".to_string(),
        "buildusername".to_string(),
    );
    println!("Built user: {:?}", built_user);

    // Create instances from other instances
    let user2: User = User {
        email: String::from("user2email@example.com"),
        ..user1
    };
    println!("User 2 is: {:?}", user2);

    // Tuple Structs
    let colour_black: Colour = Colour(0, 0, 0);
    println!(
        "Colour black is: ({}, {}, {})",
        colour_black.0, colour_black.1, colour_black.2
    );
    let colour_white: Colour = Colour(255, 255, 255);
    println!(
        "Colour white is: ({}, {}, {})",
        colour_white.0, colour_white.1, colour_white.2
    );

    let origin: Point = Point(0, 0, 0);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Unit-Like Struct
    let _subject: AlwaysEqual = AlwaysEqual;
}

// === Bank Account Struct
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    pub fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}.",
            self.balance, self.owner
        );
        self.balance -= amount;
    }

    pub fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}.",
            self.owner, self.balance
        );
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
#[derive(Debug)]
struct Colour(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit-Like struct
struct AlwaysEqual;

// Variables and Mutability
#[allow(dead_code)]
pub fn variables_immutability() {
    // let a: i32 = 5;
    let mut a: i32 = 5;
    println!("The value of a is: {}", a);
    // a = 10;
    a = 10;
    println!("The value of a is now: {}", a);
}

// Tuple
#[allow(dead_code)]
pub fn tuples() {
    let rectangle: (u32, u32) = (200, 5);
    println!("Rectangle is: {:?}", rectangle);
}

// Return a struct from a function
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
