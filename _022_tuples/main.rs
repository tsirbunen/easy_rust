// This is an empty function === it is an empty tuple
fn _this_is_tuple() {}
// same as this: returns () === empty tuple
fn _empty_tuple() -> () {}

fn main() {
    // Tuples can contain elements of different types
    let random_tuple = (8, "WORD", ['A', 'B']);
    println!("random_tuple is {:?}", random_tuple);
    // Note: access elements with .0, .1, .2, etc.
    println!("random_tuple.0 is {} = 8", random_tuple.0);

    // Destructuring tuples
    let (a, _b) = (333, 567);
    println!("a is {} = 333", a);

    let some_vec = vec!["one", "two", "three"];
    let (_element_1, _element_2, element_3) = (some_vec[0], some_vec[1], some_vec[2]);
    println!("element_3 is {} = three", element_3);
}
