
pub fn run() {
    let age: u8 = 55;
    let knows_person: bool = true;

    if age >= 21 && age < 50 || knows_person {
        println!("Good.");
    } else if age >= 51 {
        println!("Super good.");
    } else {
        println!("Bad.");
    }

    // Shorthand if
    let is_age: bool = if age >= 21 { true } else { false };

    println!("is_age: {}", is_age);
}