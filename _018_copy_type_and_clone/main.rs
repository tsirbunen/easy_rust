fn print_number(number: u8) {
    println!("Number: {number}");
}

fn print_string(string: String) {
    println!("String: {string}");
}

fn string_length(string: &String) {
    println!("Length: {}", string.split_whitespace().count())
}

fn main() {
    let integer = 3;
    // Why does this work? Earlier with String variable it did not!
    // This is because the primitive type integer implements the Copy trait
    // meaning that below (instead of giving ownership of the actual variable to the
    // function we are only COPYING the value of the variable and
    // passing that on)
    // So the integer variable that we give away here is a NEW variable
    // and we still hold the ownership to the original variable
    print_number(integer);
    // Therefore we can do this again and again (make a new copy)
    print_number(integer);
    print_number(integer);

    // But the above would / did not work for String because String does not
    // implement the Copy trait. Instead it implements the .clone() method.
    let string = String::from("India");
    // Instead of the earlier, here we force rust to make a copy of the variable
    // because it does not do that by itself (to copy Strings be default might be costly
    // because the variable value of String could be like 100 pages of a book!).
    // So here we give the function a copy (a CLONE) of the variable's value.
    print_string(string.clone());
    // And therefore we can call the function again and again
    // (by making a new copy every time)
    print_string(string.clone());
    print_string(string.clone());

    // However, the clone method for string should be used only when we really need it:
    let mut growing_string = String::new();
    for _ in 0..100 {
        growing_string.push_str("word ");
        // Note: Instead of creating here a clone (copy) of the string each time,
        // we give a reference to the same string every time
        string_length(&growing_string);
    }
}
