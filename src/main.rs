mod sample_1;

fn main() {
    println!("Hello, world! main");

    run_sample_1(false);
}

fn run_sample_1(show_sample: bool) {
    if show_sample {
        sample_1::run_sample_1();
    }
}
