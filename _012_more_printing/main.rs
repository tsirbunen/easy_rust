fn main() {
    // \t for tab and \n for new line
    println!("\tTab in start,\nnew line.");

    // Multiple lines with "" (note the starting spaces!)
    println!(
        "First line no spaces
second line with no spaces
    third line with spaces"
    );

    // Escaping /
    println!("Escape slash \\ like in \\n");

    // Too many quotes -> r#"..."#
    println!(r#"Can have " in here"#);
    println!(r##"Double #s allow even # in here"##);
    let expression = r#"Something "weird""#;
    println!("{}", expression);

    // Add printing order
    println!("{0} = F, {1} = S, {1} = S, {0} = F", "F", "S");

    // Printing named
    println!(
        "City is {city}, year is {year}",
        year = 1999,
        city = "Seoul"
    );
}
