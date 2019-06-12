pub fn run() {
    println!("Hello from print.rs file");

    println!("Number {}", 1);

    // Basic formatting
    println!("{} is from {}", "Devashish", "Nasik Road");

    // Positional Arguements
    println!(
        "{0} is from {1} and {0} loves to {2}",
        "Devashish", "Nasik Road", "Code"
    );

    // Named Arguements
    println!(
        "{name} is from {location}",
        name = "Devashish",
        location = "Nasik Road"
    );

    // Placeholder traits
    println!(
        "Binary: {:b} Hex: {:x} Octal: {:o} Decimal: {}",
        10, 10, 10, 10
    );

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
