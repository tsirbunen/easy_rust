fn main() {
    let secret = 5;
    // No brackets needed for "if" or "else" in rust
    if secret == 7 {
        println!("It's 7!")
    } else if secret == 6 {
        println!("It's 6!")
    } else {
        println!("It's not 6 or 7!")
    }

    if secret != 4 && secret <= 10 {
        println!("It's not 4 and it's less than or equal to 10!")
    } else if secret % 4 == 0 {
        println!("It's divisible by 4!")
    }

    // The "switch" in rust is called "match"
    let the_number: u8 = 88;
    match the_number {
        // This is called the first "arm"
        0 => println!("It's 0!"),
        // "Arms" are separated by commas (not semicolons)
        1 => println!("It's 1!"),
        2 => println!("It's 2!"),
        // All possible values must be covered on match!
        _ => println!("It's {}!", the_number),
        // Here we do not have a semicolon because we are not assigning a value
    }

    let minimum: u8 = 4;
    // You can also assign a value to a variable with a match
    let maximum: u8 = match minimum {
        2 => 6,
        _ => 20,
        // Here we need a semicolon because we are assigning a value
    };
    println!("Maximum is {}", maximum);

    let sky = "cloudy";
    let temperature = "warm";
    // Matching multiple values using a tuple
    match (sky, temperature) {
        ("cloudy", "warm") => println!("It's a warm, cloudy day!"),
        ("rainy", "cold") => println!("It's a cold, rainy day!"),
        _ => println!("It's {} and {} today!", sky, temperature),
    }

    let three_items: (u8, u8, u8) = (11, 12, 13);
    match three_items {
        // You can match one at a time, it will execute only
        // the first arm that matches!
        (f, _, _) if f > 10 => println!("first > 10!"),
        (_, s, _) if s > 10 => println!("second > 10!"),
        (_, _, t) if t > 10 => println!("third > 10!"),
        _ => println!("None > 10!"),
    }

    let first = 10;
    let _second = match first {
        10 => 20,
        // You must return the same type of value in all arms!
        // _ => "Not 10!",
        _ => 0,
    };

    let example_with_long_name = 4;
    match example_with_long_name {
        // We can rename the variable with possibly a long name
        // and then use this new name in the arm and use a different short name
        // for each arm (if we want)
        short @ 4 => println!("It's {}!", short),
        other_short @ 5 => println!("It's {}!", other_short),
        _ => println!("It's not 4 or 5!"),
    }
}
