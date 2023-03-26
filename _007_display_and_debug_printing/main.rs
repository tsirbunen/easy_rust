fn main() {
    // Does not print, because `()` cannot be formatted with the default formatter
    // println!("Does not print: {}", ());

    // -> We need debug printing
    // We can try "{:?}" or pretty-printing {:#?}
    println!("Does print: {:?}", ());
    println!("Does print: {:#?}", ());
}
