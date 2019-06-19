// Structs allows us to define custom datatypes


// Traditional Struct
struct Person {
    age: u8,
    name: String,
}

// Tuple Struct
struct Date(u8, u8, u16);

impl Person {
    fn set_details(n: &str, a: u8) -> Person {
        Person {
            age: a,
            name: n.to_string(),
        }
    }
    fn display(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
    fn set_name(&mut self, n: &str) {
        self.name = n.to_string();
    }
}

pub fn run() {
    let mut p1 = Person {
        age: 21,
        name: "Devashish".to_string(),
    };

    println!("Person: {} {}", p1.name, p1.age);

    let mut d = Date(15, 7, 1998);
    println!("Date: {}-{}-{}", d.0, d.1, d.2);

    let mut p2 = Person::set_details("Deva", 19);
    p2.set_name("Devashish Katoriya");
    println!("Person: {}", p2.display());
}