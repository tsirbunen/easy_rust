fn main() {
    /*
    String types:
    1. &str
        - simple
        - very fast
        - the ampersand (&): the size of str is dynamic (changes), so we need the
          pointer to that data (&)
    2. String
        - a little slower (because it is always only a pointer to data in heap)
        - the size of the data actually changes but the pointer to that data
          always has the same size
        - has more features?
    */

    let _simple_string: &str = "simple string";

    // String::from
    let _more_complicated_string_1: String = String::from("Some source");
    // "...".to_string()
    let _more_complicated_string_2: String = "Something 서태지".to_string();

    // format macro: gives you a string
    let combined = format!("one {} - two {} - three {}", 1, 2, 3);
    println!("{}", combined);
    // This would not work, the variable needs to be inside ""
    // println!(combined);

    // When there is a "from" there is an "into"
    let word = String::from("66");
    // This would not work, cannot go from integer to string like this
    // let word = String::from(66);
    println!("Word is {}", word);
    // This would not work because rust does not know into what
    // let phrase = "42352352525".into();
    // Specifying a type helps
    let phrase: String = "42352352525".into();
    println!("Phrase is {}", phrase);
}
