fn main() {
    // Signed integers
    // i8, i16, i32, i64, i128, isize

    // Unsigned integers
    // u8, u16, u32, u64, u128, usize

    // 8 => 8 bits = 1 byte
    // 64 = 8 * 8 bits => 8 bytes

    // 32-bit processor computer -> usize = u32
    // 64-bit processor computer -> usize = u64

    // char "A" has unicode number 1
    // chinese "友" has unicode number 21451
    // most used characters have unicode <= 256 (can fit to u8)

    let _first_letter: char = 'A';
    let _space: char = ' '; // A space inside ' ' is also a char
    let _other_language_char: char = 'Ꮔ'; //  Cherokee
    let _cat_face: char = '😺'; // Emojis are chars too

    // If you don't choose type, rust selects i32 by default for numbers
    let my_number = 100; // rust thinks this is i32
    println!("my number {}", my_number); // my number 100
    println!("my number {} as i32", my_number as i32); // my number 100
    println!("my number {} as u32", my_number as u32); // my number 100

    // This won't work, because i32 (that rust thinks the number is)
    // can be really big, much higher than there are unicode numbers
    // so i32 -> char is not safe.
    // println!("my number {} as char", my_number as char); // my number 100
    // But double casting first to u8 and then to char works because it is guaranteed
    // that there exists a char for every u8 number!
    println!("my number {} as char", my_number as u8 as char); // my number 100

    let number_with_type: u8 = 100;
    println!("number with type {} as char", number_with_type as char); // number with type 100
}
