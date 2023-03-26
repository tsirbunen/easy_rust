fn main() {
    // This is not allowed (rust does not know the type):
    // let uninitialized_variable_1;

    // This compiles, but we still get a warning about a possibly
    // uninitialized variable:
    let _uninitialized_variable_2: u8;

    // This works, because we have BOTH the if and the else branches
    // so it is guaranteed that the variable will be initialized
    let uninitialized_variable_3: u8;
    if true {
        uninitialized_variable_3 = 5;
    } else {
        // If we did not have this else branch, rust would not compile
        // complaining that the variable might be uninitialized
        uninitialized_variable_3 = 6;
    }
    println!("Value {}", uninitialized_variable_3);
}
