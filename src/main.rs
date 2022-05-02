// #[allow(unused_imports)]
mod sample_1;
#[allow(unused_imports)]
mod sample_2;
mod user;
// #[allow(unused_imports)]
// use crate::sample_2::MySample;
use sample_2::*;
#[allow(unused_imports)]
use crate::user::*;

fn main() {
    println!("Hello, world! main");
    run_sample_1(true);
    create_user(true);
}

fn run_sample_1(show_sample: bool) {
    if show_sample {
        sample_1::run_sample_1();
        let variable = MySample { number: 1};
        println!("struct number created: {}", variable.number);
    }
}

fn create_user(create_user: bool) {
    if create_user {
        let user = User::new(String::from("someone"));
        println!("user created: {} ", user.username);
    }
}
