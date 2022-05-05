

fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}

fn plus_one(a: i32) -> i32 {
    return a+1
}

fn plus_two(a: i32) -> i32 {
    return a+2;
}

/* Closures */
pub fn using_closures() {
    let x = 2;

    println!("use function {}", get_square_value(x));

    let square = |i: i32| -> i32 {
        i * i;
    };
    let square_2 = |i| i*i;
    println!("closures {}", square(x));
    println!("closures {}", square2(x));

    let x_square = |i: i32| -> i32 { i * i }(x);
    let x_square_2 = |i| -> i32 { i * i }(x); // ⭐️ The return type is mandatory.
    println!("closures {}", x_square);
    println!("closures {}", x_square_2);

}

fn get_square_value(i: i32) -> i32 {
    return i * i;
}