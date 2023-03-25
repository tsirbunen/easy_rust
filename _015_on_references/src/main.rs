// we are trying to return a reference to a String
fn give_me_india() -> &str {
    // variable india is a String
    let india = String::from("INDIA");
    // variable ref_india is a reference to a String
    let ref_india = &india;
    // here in the function the reference WOULD work
    // if we could run this file
    println!("ref_india is {}", ref_india);

    // The file does not run because this does not work:
    // here we are trying to return a reference to a String
    ref_india
    // The problem:
    // This function OWNS the variable india. When we leave this function,
    // the variable india will die. Therefore we cannot return a reference
    // to it because the reference is no longer when we return!
}

fn main() {
    let country = give_me_india();
}
