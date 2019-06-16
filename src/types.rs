
// Integer: i8, i16, i32, i64, i128
// Float: f64, f128
// char
// bool
// Arrays, Vectors
// Tuples

pub fn run() {
    // Default is i32
    let x = 23;

    // Default is f64
    let y = 3.14;

    // Explicit declaration
    let z: i64 = 32482387357;

    println!("x, y, z are {} {} {}", x, y, z);

    // Suppress unused var warning
    let _p: i32 = 123;

    // Finding MAX value
    println!("Max i32 is {}", std::i32::MAX);
    println!("Max i64 is {}", std::i64::MAX);

    // Boolean declaration
    let flag: bool = true;
    println!("Boolean is {}", flag);

    let ch: char = 'y';
    println!("Char is {}", ch);
}