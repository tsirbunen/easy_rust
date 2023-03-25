fn give_eight() -> u32 {
    // Because it is "8" and not "8;" rust returns the 8 here
    8
    // Alternatively you could do this:
    // return 8;
    // But not this:
    // 8;   <- this would return "()" which is nothing (unit type)
}

// Here we could return -> ()
// fn multiply(first: i32, second: i32) -> () {
fn multiply(first: i32, second: i32) {
    let result = first * second;
    println!("{} * {} = {}", first, second, result);
}

fn main() {
    // Here "!" tells that println is a macro
    // macros = super functions that write code
    println!("HELLO!");

    println!("Numbers {} and {}", 2, 5);
    println!("Eight: {}", give_eight());

    // Here rust shows the names of the variables, we did not write them
    multiply(3, 7);
    let f = 4;
    let s = 7;
    multiply(f, s);
}
