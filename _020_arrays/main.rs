fn main() {
    /*
        Arrays
        - must not change their size
        - must contain elements of a single same type
        - are very fast
        - type is [T; N] where T = type and N = number of elements
    */
    let number_array: [&str; 3] = ["one", "two", "three"];
    // Print all to the same row
    println!("Numbers: {:?}", number_array);
    // Print each element on its own row
    println!("Numbers: {:#?}", number_array);

    // Create an array of repeating elements (note the ";")
    let words_array: [&str; 3] = ["word"; 3];
    println!("Words: {:?}", words_array);

    let smalls: [u8; 6] = [0, 1, 2, 3, 4, 5];
    println!("Smalls all: {:?}", smalls);
    // Here we need the "&" when getting a slice of the array
    println!("Smalls 1-3: {:?}", &smalls[1..4]);
    println!("Smalls 1-end: {:?}", &smalls[1..]);
    println!("Smalls start-4: {:?}", &smalls[..5]);
    println!("Smalls start-end: {:?}", &smalls[..]);
    // Above is the "normal" EXCLUSIVE end of range
    // One can also have INCLUSIVE end of range with "="
    println!("Smalls 1-3: {:?}", &smalls[1..=3]);
}
