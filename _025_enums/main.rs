enum Fruits {
    Apple,
    Mango,
    Cherry, // comma here is optional
}

fn give_me_fruit(color: &str) -> Fruits {
    match color {
        "green" => Fruits::Apple,
        "yellow" => Fruits::Mango,
        "red" => Fruits::Cherry,
        _ => panic!("I don't know this color"),
    }
}

enum ThingInTheSky {
    Moon(String),
    Sun(String),
}

fn get_item_in_the_sky(time_of_day: &str) -> ThingInTheSky {
    match time_of_day {
        "night" => ThingInTheSky::Moon(String::from("MOON")),
        _ => ThingInTheSky::Sun(String::from("SUN")),
    }
}

fn check_item(item: &ThingInTheSky) {
    match item {
        ThingInTheSky::Moon(description) => println!("{}", description),
        ThingInTheSky::Sun(description) => println!("{}", description),
    }
}

enum Mood {
    Happy,       // This is 0 by default
    Sleepy = 14, // Here we set our own value
    Sad = 33,
}

fn get_mood_level_with_repeating(mood: &Mood) -> u8 {
    match mood {
        // Here we repeat "Mood::" every time
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::Sad => 0,
    }
}

fn get_mood_level_simplified(mood: &Mood) -> u8 {
    // If we "import" Mood::, we need not repeat it
    use Mood::*;
    match mood {
        Happy => 10,
        Sleepy => 6,
        Sad => 0,
    }
}

enum SomeKindOfNumber {
    I8(i8),
    U8(u8),
}

fn get_some_number(input: i32) -> SomeKindOfNumber {
    let number = match input.is_positive() {
        true => SomeKindOfNumber::U8(input as u8),
        false => SomeKindOfNumber::I8(0),
    };
    number
}

fn main() {
    /*
    - struct when you want one thing AND another thing
      (structs are for a collection of things)
    - enum when you want one thing OR another thing
      (enums are for a selection of options)
    */
    let fruit_color = "green";
    let _fruit = give_me_fruit(fruit_color);

    let time_of_day = "night";
    let item = get_item_in_the_sky(time_of_day);
    check_item(&item);

    let mood = Mood::Happy;
    let level_repeating = get_mood_level_with_repeating(&mood);
    println!("{}", level_repeating);
    let level_simplified = get_mood_level_simplified(&mood);
    println!("{}", level_simplified);

    // We can also import an enum in the main function
    use Mood::*;
    let mood_now = Happy;
    // And get the enum value
    println!("Mood now {} = 0", mood_now as u32);
    let mood_at_night = Sleepy;
    // We can set our desired value to enum options
    println!("Mood at night {} = 14", mood_at_night as u32);

    // Normally vectors can only contain one type of members like
    let one: u8 = 1;
    let two: u8 = 2;
    let three: i8 = 3;
    // You can do this:
    let _vec = vec![one, two];
    // But this would not work (three is of different type):
    // let vec = vec![one, two, three];
    // Custom enums help here as they are all same type (the enum):
    let _vec = vec![SomeKindOfNumber::U8(one), SomeKindOfNumber::I8(three)];
}
