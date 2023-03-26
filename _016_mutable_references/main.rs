fn main() {
    // The variable has to be mutable in the first place
    let mut target = 3;
    // Only then we can have a mutable reference to it
    let target_ref = &mut target;

    // If we want to change the value of the original variable target,
    // we cannot do this (add type integer to type reference):
    // target_ref = target_ref + 1;
    // target_ref is not the variable nor a copy of it but a reference to it!
    // To mutate the value in the original variable, we need to dereference
    // the reference with the * operator:
    *target_ref = *target_ref + 1;
    *target_ref += 1;

    println!("Target is now {} = 5", target);

    // & -> reference
    // * -> dereference

    /*
    You can have
    - either as many immutable references as you like
      (but no simultaneous mutable references)
    - or a single mutable reference
      (but no simultaneous immutable references)
    */

    // Here we have two immutable references to the same mutable variable
    let mut many = "MANY";
    let _many_ref_1 = &many; // immutable reference
    let _many_ref_2 = &many;
    // We would not be able to have a mutable reference in addition to the immutable ones
    // let _many_ref_3 = &mut many; // mutable reference would not do
    println!("Many {}", many);
    println!("Many ref 1 {}", _many_ref_1);
    // We can change the value of the mutable variable
    many = "MANY MUTATED";
    // But then we cannot access the immutable variable anymore so the below does not work
    // println!("Many ref 1 {}", _many_ref_1);
    println!("Many {}", many);

    // Here we have a single mutable variable
    let mut single = "SINGLE";
    // And a single mutable reference to it
    let single_ref = &mut single; // mutable reference

    // We would not be able to have an immutable reference in addition to the mutable one
    // let _single_ref_2 = &single; // immutable reference would not do
    *single_ref = "SINGLE MUTATED";
    println!("Single {}", single);

    // Shadowing example
    let country = "INDIA";
    let country_ref = &country;
    let country = "CHINA";
    println!("Shadowing country is {} = CHINA", country);
    println!("Original country is {} = INDIA", country_ref);
}
