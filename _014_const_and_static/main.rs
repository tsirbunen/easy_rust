fn main() {
    // Must give a TYPE to const and name must ne in UPPERCASE
    const CONSTANT_VALUE: u8 = 111;
    // This would not do:
    // const CONSTANT_VALUE = 111;

    println!("Constant {}", CONSTANT_VALUE);

    // This will not work. You cannot shadow constant values!
    // let CONSTANT_VALUE = 444;

    // static variable can act as a global, has a fixed memory location
    static _SEASONS: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];
}
