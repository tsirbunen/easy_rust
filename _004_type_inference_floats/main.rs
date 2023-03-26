fn main() {
    // Type inference = rust will guess the type (rather decide):
    let _my_number = 23;
    // Integers are always 132 if not specified.

    // Force the type
    let _other_number = 23u32; // 23 is type u32
    let _another_number = 23_u32; // 23 is type u32
    let _another_number = 23____u32; // 23 is type u32
    let _yet_another: u32 = 23;

    let _some_test_number = 1_000_000;
    let _another_test_number = 1_3_5; // 135

    // FLOATS:
    // f32 and f64 (32 and 64 bits); f64 is the default
    let _f_1 = 5.0;
    let _f_2 = 5.; // rust interprets this as 5.0

    // if not specified, rust decides f64:
    let _f1 = 3.4;

    // If one is specified to f32 but the other is not AND you sum them,
    // rust decides the other is also f32:
    let f2: f32 = 22.3;
    let f3 = 2.9;
    let _sum1 = f2 + f3;

    // This won't work, because no implementation for `f64 + f32`:
    let _f4: f64 = 3.3;
    let _f5: f32 = 7.9;
    // let sum2 = _f4 + _f5;
    // println!("{}", sum2);
}
