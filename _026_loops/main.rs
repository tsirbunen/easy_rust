fn main() {
    let mut counter_outer: u8 = 0;
    let mut counter_inner: u8 = 0;

    'outer_loop: loop {
        counter_outer += 1;
        println!("Counter outer: {counter_outer}");
        if counter_outer > 2 {
            'inner_loop: loop {
                counter_inner += 1;
                println!("\tCounter inner: {counter_inner}");
                if counter_inner == 3 {
                    counter_inner = 0;
                    break 'inner_loop;
                }
            }
        }
        if counter_outer == 5 {
            break 'outer_loop;
        }
    }

    let mut rounds = 1;
    while rounds < 4 {
        println!("Rounds: {rounds}");
        rounds += 1;
    }

    for number in 0..3 {
        println!("0, 1 or 2: {number}");
    }
    for number in 0..=3 {
        println!("0, 1, 2 or 3: {number}");
    }

    // We can RETURN a value from loop by breaking it and set it to a variable
    let mut count = 0;
    let sum = loop {
        count += 1;
        if count % 3 == 0 {
            break count;
        }
    };
    println!("Sum {sum}");
}
