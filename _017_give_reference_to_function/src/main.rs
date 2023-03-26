//// CASE 1:
// fn print_country(country_name: String) {
//     // 3. We can print the variable value here
//     println!("Country: {}", country_name);
//     // 4. But we are not returning anything so once we leave this function
//     //    the variable country_name will die and will not be available anymore
// }

// fn main() {
//     // 1. Here we create country that is the owner of the String "India"
//     let country = String::from("India");
//     // 2. We give the variable country to the function print_country
//     //    and therefore do not own it anymore
//     //    (the value "moves" to the function)
//     print_country(country);

//     // 5. Because we gave the variable country to the function print_country
//     //    we lost ownership to it and because the variable died when we left
//     //    that function, it is no longer available to us here and therefore
//     //    the below command would not work.
//     // print_country(country);
// }

// // CASE 2:
// // 3. Now the function receives a reference to our variable so it only borrows
// //    the variable and does not take ownership of it
// fn print_country_ref(country_name: &String) {
//     println!("Country: {}", country_name);
//     // 4. When we leave this function, the variable does not die but lives on
// }

// fn main() {
//     // 1. Here we create variable country that is the owner of the String "India"
//     let country = String::from("India");
//     // 2. Then instead of earlier, we give the printing function only a
//     //    reference to the variable country (so we will keep the ownership to the variable
//     //    and only allow the function to borrow it)
//     print_country_ref(&country);

//     // 5. Because the variable did not die in the printing function it is still available
//     print_country_ref(&country);
// }

// CASE 3:

// 3. This function receives a mutable reference (so it can mutate the variables
//    referred value but does not take ownership)
fn add_india(country_name: &mut String) {
    // 4. The value (of the reference that is owned by the caller) is mutated
    country_name.push_str("-India");
    println!("Combined: {}", country_name);
}

fn main() {
    // 1. Here we create variable country that is the owner of the String "China"
    //    and this variable is now mutable
    let mut country = String::from("China");
    // 2. We give a reference to this variable to add_india function and this
    //    reference is a mutable reference (so we would not be able to give other mutable
    //    references or immutable references to it)
    add_india(&mut country);
    // 5. Because we only gave reference to the variable it is still available
    add_india(&mut country);
    // 6. And because the reference was mutable and we mutated the value
    //    its value has indeed changed
    println!("Current: {} = China-India-India", country);
}
