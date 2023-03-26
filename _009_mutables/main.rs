fn main() {
    let immutable = 2;
    // This will not work, because cannot mutate immutable variable
    // immutable = 44;
    println!("Immutable {immutable} stays that");

    // Mutables can be mutated
    let mut mutable = 3;
    mutable = 44;
    println!("Mutable {mutable} was mutated");

    // But even mutables do not tolerate type change
    // So cannot do this (assigning a value of different type to SAME variable):
    // mutable = "Some string";

    // But could do this (SHADOWING):
    // And here the variable "mutable" is different from the one above
    let mutable = "Some string";
    println!("Mutable {mutable} was mutated");
}
