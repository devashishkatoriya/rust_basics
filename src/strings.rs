pub fn run() {
    let mut hello = String::from("Hello "); // String::from() is must for push method

    // Get length
    println!("Length: {}", hello.len());

    // Append char
    hello.push('W');

    // Appends string
    hello.push_str("orld!");

    // Finding capacity
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace 'rld' with 'abc' {} ", hello.replace("rld", "abc"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}