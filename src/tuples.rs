// Collection of Heterogenous elements
// Max 12 elements

pub fn run()
{
    let person: (&str, &str, i16) = ("Devashish", "Katoriya", 21);

    println!("{} {} is {}", person.0, person.1, person.2);
}