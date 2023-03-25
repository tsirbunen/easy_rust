fn main() {
    let item = 555;
    println!("Item is {} = 555", item);

    // Here we SHADOW the variable "item"
    // The original variable lives on, but we can't see it
    // because we have blocked it with this new variable of same name
    let item = "WORD";
    println!("Item is {} = WORD", item);

    let original = 0;
    println!("Original is {} = 0", original);
    {
        // Shadowing inside the bloc multiple times
        let original = 44;
        // We do this when we are not really interested in the variable (name)
        // Without shadowing we would need to have new names for each line here
        let original = original * 2;
        let original = original - 5;
        println!("Original is {} = 83", original);
    }
    // Here we can see the original value
    println!("Original is {} = 0", original);
}
