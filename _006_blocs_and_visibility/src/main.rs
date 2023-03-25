fn main() {
    // Declaring a variable with "let" binding
    let _unused = 'U';

    let visible_number = 33;

    // {} represents a code bloc
    {
        let _not_visible_outside_bloc = 66;
    }

    println!("Can see {}", visible_number);
    // println!("Can NOT see {}", _not_visible_outside_bloc);

    // Here the {} is a little like a function
    let bloc_number = {
        let insider = 44;
        // Because "insider" and not "insider;", this returns 44
        insider
    }; // <- is executed
    println!("Insider {} = 44", bloc_number);

    let empty = {
        let mut _inside_number = 3;
        _inside_number = _inside_number + 8;
    };
    // Here the ":?" tells that there is necessarily no value
    println!("Empty {:?} = 44", empty);
}
