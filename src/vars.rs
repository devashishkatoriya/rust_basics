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

    // Define constant
    const ROLL: i32 = 21;

    println!("Roll is {}", ROLL);

    // Assigning multiple vars
    let (my1, my2) = ("Test1", 23);
    
    println!("my1 is {}, my2 is {}", my1, my2);
}
