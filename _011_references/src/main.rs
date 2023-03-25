fn main() {
    /*
    How to keep memory?
    1. STACK
        - faster
        - needs to know the exact size of the item stored:
          like "let target: u8 = 2;" because u8 -> 1 byte which is ok
    2. HEAP
        - slower

    When rust does not know the exact size of the variable, it will
    store a REFERENCE = pointer to the actual value in the heap and place
    the pointer (which has a known size) to stack (with some extra data).
    */

    let friend = "FRIEND";
    // Reference to friend
    let first_ref_friend = &friend;
    // Reference to a reference to a friend
    let second_ref_friend = &first_ref_friend;
    let another_second_ref = &&friend;
    println!("{} = true", second_ref_friend == another_second_ref);
    // But this won't work because cannot compare `&4&str` with `str`
    // println!("{} = true", second_ref_friend == friend);

    let integer = 4;
    let integer_ref = &&integer;
    // This will not work because cannot compare `{integer}` with `&{integer}`
    // println!("{} = true", integer == integer_ref);
    // But * counteracts & (one * takes one & away)
    println!("{} = true", integer == **integer_ref);
}
