
pub fn find_t_shirt_size(t_shirt_width: i32) {
    let t_shirt_size = match t_shirt_width {
        16 => "S",          // size 16
        17 | 18 => "M",     // size 17, 18
        19 ..= 21 => "L",   // size from 19 to 21
        22 => "XL",
        _ => "Not Available"
    };
    println!("Current t-shirt size: {}", t_shirt_size);
}

#[allow(dead_code)]
fn paper_check(marks_paper_a: u8, marks_paper_b: u8) {
    // let marks_paper_a: u8 = 25;
    // let marks_paper_b: u8 = 30;
    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard"
    };
    println!("{}", output);
}

#[allow(dead_code)]
#[allow(unused_labels)]
#[allow(unreachable_code)]
pub fn do_nesting_label_loop() {
    // The label can be any string
    'outer_label: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer_label;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}