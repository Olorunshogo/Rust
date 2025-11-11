
pub mod assignment_1 {
    // Convert temperatures between Fahrenheit and Celsius
    /// Make both the fahrenheit_to_celsius and celsius_to_fahrenheit to support decimal (f32)
    /// and probably display up to just 2 decimal places
    
    pub fn fahrenheit_to_celsius(x: u32) -> f64 {
        // let celsius: f64 = (x - 32.0) * (5.0 / 9.0);
        // println!("The value of {x}°F is: {}°C", celsius);

        let celsius: f64 = (x as f64 - 32.0) * (5.0 / 9.0);
        println!("The value of {x}°F is: {:.3}°C", celsius);
        return celsius;
    }
    
    pub fn celsius_to_fahrenheit(x: f64) -> f64 {
        let fahrenheit: f64 = (x * 1.80) + 32.0;
        println!("The value of {x}°C is: {:.1}°F", fahrenheit);
        return fahrenheit;
    }

    // Generate the nth Fibonacci number.
    pub fn fibonacci_number(n: u32) -> u32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }
    
        let mut a: u32 = 0;
        let mut b: u32 = 1;
    
        for _ in 2..=n {
            let temporary_var = a + b;
            a = b;
            b = temporary_var;
        }
    
        println!("The {}th Fibonacci number is: {}.", n, b);
        return b;
    }

    // Generate the nth Fibonacci sequence
    pub fn fibonacci_sequence(n: u32) {
        let mut a: u32 = 0;
        let mut b: u32 = 1;
    
        println!("n    Fn");
    
        for i in 0..n {
            println!("{:<4} {}", i, a);
            let temporary_var = a + b;
            a = b;
            b = temporary_var;
        }
    }

    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    pub fn lyrics_loop_testing() {
        let lyrics_testing: [&str; 12] = [
            "A partridge in a pear tree",
            "Two turtle doves and",
            "Three french hens",
            "Four calling birds",
            "Five golden rings",
            "Six geese a-laying",
            "Seven swans a-swimming",
            "Eight maids a-milking",
            "Nine ladies dancing",
            "Ten lords a-leaping",
            "Eleven pipers piping",
            "Twelve drummers drumming",
        ];

        let positions: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for (index, _day) in positions.iter().enumerate() {
            println!("Verse {}", positions[index]);
            println!(
                "On the {} day of Christmas, my true love sent to me",
                lyrics_testing[index]
            );

            for j in (0..=index).rev() {
                if j > 0 {
                    println!("{}", lyrics_testing[j]);
                    // println!("{}", lyrics_testing[0]);
                    // println!("");
                } else {
                    // println!("");
                }
            }

            println!("{}", lyrics_testing[0]);
            println!("");
        }
    }

    /// Work on the implementation to print the particular lyrics of a particular day
    fn lyrics_loop() {
        let christmas_lyrics: [&str; 12] = [
            "A partridge in a pear tree",
            "Two turtle doves and",
            "Three french hens",
            "Four calling birds",
            "Five golden rings",
            "Six geese a-laying",
            "Seven swans a-swimming",
            "Eight maids a-milking",
            "Nine ladies dancing",
            "Ten lords a-leaping",
            "Eleven pipers piping",
            "Twelve drummers drumming",
        ];

        let ordinals: [&str; 12] = [
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth", "eleventh", "twelfth",
        ];

        let positions: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        for (index, day) in ordinals.iter().enumerate() {
            println!("Verse {}", positions[index]);
            println!("On the {} day of Christmas, my true love sent to me", day);

            for j in (0..=index).rev() {
                if j > 0 {
                    println!("{}", christmas_lyrics[j]);
                }
            }

            println!("{}", christmas_lyrics[0]);
            println!("");

            // for j in (0..=index).rev() {
            //   // Add "and" before the first item if it's not the first day
            //   if j == 0 && index != 0 {
            //       println!("and {}", christmas_lyrics[j]);
            //   } else {
            //       println!("{}", christmas_lyrics[j]);
            //   }
            // }
        }
    }
}

pub mod assignment_2 {
    // Step1: Define the Size Enum
    #[derive(Debug)]
    pub enum CoffeeSize {
        Small,
        Medium,
        Large,
    }

    // Step 2: Define the order Struct
    #[derive(Debug)]
    pub struct CoffeeOrder {
        customers_name: String,
        shot_size: CoffeeSize,
        is_drink_iced: bool,
        shot_quantity: u32,
    }

    // Step 3: Add Methods to CoffeeSize
    impl CoffeeSize {
        pub fn base_price(&self) -> f32 {
            match self {
                CoffeeSize::Small => { 3.50 },
                CoffeeSize::Medium => { 4.50 },
                CoffeeSize::Large => { 5.50 },
            }
        }

        pub fn ounces(&self) -> u8 {
            match self {
                CoffeeSize::Small => 8,
                CoffeeSize::Medium => 12,
                CoffeeSize::Large => 16,
            }
        }
    }

    // Step 4: Add Methods to CoffeeOrder
    impl CoffeeOrder {
        // Create a new coffee order
        pub fn new(customers_name: &str, size: CoffeeSize) -> Self {
            CoffeeOrder {
                customers_name: customers_name.to_string(),
                shot_size: size,
                is_drink_iced: false,
                shot_quantity: 2,
            }
        }

        // Make the coffee iced
        pub fn make_iced(&mut self) {
            self.is_drink_iced = true;
        }

        // Add extra espresso shots
        pub fn add_espresso_shot(&mut self, count: u32) {
            self.shot_quantity += count;
        }

        // Calculate total price
        pub fn calculate_total_price(&self) -> f32 {
            let base_price = self.shot_size.base_price();
            let base_shots: f32 = 2.0;
            let extra_shots = if (self.shot_quantity as f32) > base_shots {
                (self.shot_quantity as f32) - base_shots
            } else {
                0.0
            };

            let total_price = base_price + (extra_shots as f32 * 0.50);
            return total_price;
        }

        // Print a friendly description
        pub fn describe(&self) {
            let drink_type = if self.is_drink_iced { "an Iced" } else { "a Hot" };
            
            println!(
                "{} ordered {:?} {} coffee ({} oz) with {} espresso shots. Total: ${:.2}",
                self.customers_name,
                drink_type,
                match self.shot_size {
                    CoffeeSize::Small => "Small",
                    CoffeeSize::Medium => "Medium",
                    CoffeeSize::Large => "Large",
                },
                self.shot_size.ounces(),
                self.shot_quantity,
                self.calculate_total_price(),
            );
        }

    }

    // Step 5: Test Everything
    pub fn run_coffee_shop_demo() {
        // First Order
        let mut order1 = CoffeeOrder::new("Jane", CoffeeSize::Medium);
        order1.make_iced();
        order1.add_espresso_shot(2);
        order1.describe();

        
        // Second Order
        let mut order2 = CoffeeOrder::new("John", CoffeeSize::Small);
        order2.add_espresso_shot(4);
        order2.describe();

        // Third Order
        let order3 = CoffeeOrder::new("Doe", CoffeeSize::Large);
        order3.describe();
    }
    
}


