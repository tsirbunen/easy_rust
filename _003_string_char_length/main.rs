fn main() {
    // rust uses '' for characters, not ""
    // "" is a string

    // All characters are always 4 bytes (max 32u = 4 * 8 bits) AS chars!
    // But in strings they take up as little space as possible (less than as chars).
    // .len() gives the size of the string in bytes (not the " usual length")
    println!("Size of string with char 'a': {} = 1 byte", "a".len());
    println!("Size of string with char 'ß': {} = 2 bytes", "ß".len());
    println!("Size of string with char '国': {} = 3 bytes", "国".len());
    println!("Size of string with char '𓅱': {} = 4 bytes", "𓅱".len());

    // .chars().count() gives the number of characters in a string (the " usual length")
    println!("Bytes of HELLO: {}", "HELLO".len());
    println!("Bytes of HELLO 𓅱: {}", "HELLO 𓅱".len());
    println!("Number of chars in HELLO: {}", "HELLO".chars().count());
    println!("Number of chars in HELLO 𓅱: {}", "HELLO 𓅱".chars().count());
}
