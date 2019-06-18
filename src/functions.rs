pub fn run() {

    // Calling function
    display_line("Hello!");

    let sum = add(5, 10);
    println!("Sum: {}", sum);

    // Closure
    let n3 = 10;
    let add_nos = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("C sum: {}", add_nos(5, 10));
}

fn display_line(data: &str) {
    println!("Data: {}", data);
}

fn add(a: i8, b: i8) -> i8 {
    return a + b;
}