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
mod sample_control_flow;

fn main() {
    println!("Hello, world! main");
    run_sample_1(false);
    create_user(false);
    check_t_shirt_size(true);
}

fn run_sample_1(show_sample_1: bool) {
    if show_sample_1 {
        sample_1::run_sample_1();
        let variable = MySample { number: 1};
        println!("struct number created: {}", variable.number);
    }
}

fn create_user(show_create_user: bool) {
    if show_create_user {
        let user = User::new(String::from("someone"));
        println!("user created: {} ", user.username);
    }
}

fn check_t_shirt_size(show_t_shirt: bool) {
    if show_t_shirt {
        sample_control_flow::find_t_shirt_size(16);
        sample_control_flow::do_nesting_label_loop();
    }
}
