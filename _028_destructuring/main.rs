#[derive(Debug)]
struct Fruit {
    name: String,
    sweetness: u8,
}

fn main() {
    let cherry = Fruit {
        name: String::from("Cherry"),
        sweetness: 3,
    };

    // We can destructure a struct
    let Fruit {
        name: n,
        sweetness: s,
    } = cherry;

    println!("name {n}, sweetness {s} = 3");
}
