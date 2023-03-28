// 1. Unit struct does not contain anything (it is empty)
// (ends with a semicolon)
struct UnitStruct;

// 2. Tuple struct has unnamed elements (only types are given)
// (ends with a semicolon)
struct Color(u8, u8, u8);

// 3. Struct with named elements with types
// (does NOT end with a semicolon)
struct Properties {
    width: u8,
    height: u8,
    color: Color, // You can have a comma here OR leave it out
}

struct Country {
    capital: String,
    population: u32,
}

fn main() {
    let _unit: UnitStruct = UnitStruct;
    let color: Color = Color(255, 0, 0);
    println!("color's first element is {:?}", color.0);

    let properties: Properties = Properties {
        width: 100,
        height: 200,
        color: Color(21, 78, 33),
    };
    println!(
        "{}, {}, {}",
        properties.width, properties.height, properties.color.0
    );

    let capital = String::from("Capital");
    let population = 1_000_000;
    let country: Country = Country {
        capital,
        population,
    };
    println!("{} has {} people", country.capital, country.population);
}
