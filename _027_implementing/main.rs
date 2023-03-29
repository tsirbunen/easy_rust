// #[] is an attribute
// #[derive(Debug)] allows us to print with {:?}
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl AnimalType {
    // Also enums can have METHODs
    fn print_type(&self) {
        match self {
            AnimalType::Cat => println!("Type: cat"),
            AnimalType::Dog => println!("Type: dog"),
        }
    }
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    // FUNCTION that returns a new instance of the "Animal"
    // A kind of "static" function that does not take the self as an input
    fn new() -> Self {
        Self {
            age: 0,
            animal_type: AnimalType::Cat,
        }
    }

    // METHOD: here we get a reference to self
    fn convert_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
    }

    fn print_type(&self) {
        match self.animal_type {
            AnimalType::Cat => println!("Cat!"),
            AnimalType::Dog => println!("Dog!"),
        }
    }
}

fn main() {
    let mut animal = Animal::new();
    animal.print_type();
    animal.convert_to_dog();
    animal.print_type();
    println!("Age: {}", animal.age);
    animal.animal_type.print_type();
}
