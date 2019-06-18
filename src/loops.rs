pub fn run() {
    let mut count: u8 = 0;

    // Infinite loop
    loop {
        println!("Count: {}", count);
        if count == 20 {
            break;
        }
        count = count + 1;
    }

    // While loop (FizzBuzz game)
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count = count + 1;
    }

    // For in range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}