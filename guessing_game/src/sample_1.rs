use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub mod sample_1 {

    fn run_sample_1() {
        guess1();

        // let input_i32: i32 = 1;
        // if input_i32 == 1 {
        //     println!("output value: {}", plus_one(input_i32));
        // }

        // println!("User {}", define_user().username);

        // calculate_length()
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }

    #[allow(dead_code)]
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }

    #[allow(dead_code)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    #[allow(dead_code)]
    fn define_user() -> User {
        return User {
            username: "someone1".to_string(),
            email: String::from("someone1@example.com"),
            active: true,
            sign_in_count: 1
        }
    }

    #[allow(dead_code)]
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    #[allow(dead_code)]
    fn guess1() {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("The secret number is: {}", secret_number);
        loop {
            println!("Please input your guess.");
            // variables by default are immutable, mut keyword will mark it as mutable
            let mut guess = String::new();

            /*  read_line: this will allow us to HANDLE USER INPUT
                expect: this will handling potential failure
             */
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
