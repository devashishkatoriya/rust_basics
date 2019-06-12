// Variables hold primitive data or references to data
// Vars are immutable by default
// Rust is block scoped language

pub fn run() {
    let name = "Devashish";
    // name = "DK"; // will not work

    println!("My name is {}", name);

    let mut name2 = "Deva";
    name2 = "Deva22";

    println!("My name is {}", name2);
}
