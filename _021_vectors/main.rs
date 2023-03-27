fn main() {
    // Vectors are less performant than arrays

    let name1 = String::from("Windy");
    let name2 = String::from("Sunny");
    let name3 = String::from("Rainy");

    // Declare a vector - CASE 1 (without type):
    // Here we do not tell rust the type, the editor guesses it
    let mut name_vec = Vec::new();
    // Running only the above code would cause error because rust would not
    // know the type of the elements in the vector (all must be of the same type)
    name_vec.push(name1);
    name_vec.push(name2);
    name_vec.push(name3);

    // As we pushed items to the vector, rust knows the type of the vector
    // and the below works
    println!("Vector {:?}", name_vec);

    // Declare a vector - CASE 1 (with type):
    let mut word_vec: Vec<String> = Vec::new();
    word_vec.push(String::from("World"));
    word_vec.push(String::from("!"));
    word_vec.push(String::from("!"));
    word_vec.push(String::from("!"));
    println!("Vector all {:?}", &word_vec[..]);
    println!("Vector part {:?}", &word_vec[2..]);
    println!("Vector part {:?}", &word_vec[..2]);

    // Declare a vector - CASE 2 (without type, use macro):
    let some_vec = vec![5, 6, 7];
    println!("Vector {:?}", some_vec);

    // Declare a vector - CASE 3 (with type, from array):
    let _vec_1: Vec<u8> = [1, 2, 3].into();
    let _vec_2: Vec<_> = [1, 2, 3].into(); // let rust guess the type

    // Vector capacity: how many elements can be stored in the vector?
    // If not set and no elements, (Vec::new()) starts with 0 capacity
    // When adding elements, the capacity is ALWAYS doubled when a capacity
    // is reached.
    // Even though we set the capacity to 5, it will be doubled to 10 when
    // adding the 6th element
    let mut char_vec: Vec<char> = Vec::new();
    let mut char_vec_with_capacity: Vec<char> = Vec::with_capacity(5);
    println!("Regular {:?} = 0", char_vec.capacity());
    println!(
        "With capacity set {:?} = 5",
        char_vec_with_capacity.capacity()
    );
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    println!("Regular {:?} = 4", char_vec.capacity());
    println!(
        "With capacity set {:?} = 5",
        char_vec_with_capacity.capacity()
    );
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    println!("Regular {:?} = 4", char_vec.capacity());
    println!(
        "With capacity set {:?} = 5",
        char_vec_with_capacity.capacity()
    );
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    println!("Regular {:?} = 8", char_vec.capacity());
    println!(
        "With capacity set {:?} = 5",
        char_vec_with_capacity.capacity()
    );
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    char_vec.push('a');
    char_vec_with_capacity.push('a');
    println!("Regular {:?} = 8", char_vec.capacity());
    println!(
        "With capacity set {:?} = 5",
        char_vec_with_capacity.capacity()
    );
}
