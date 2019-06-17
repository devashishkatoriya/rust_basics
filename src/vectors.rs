
use std::mem;

pub fn run() {

    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Add onto vector
    arr.push(6);
    arr.push(7);
    arr.push(8);

    // Pop an element
    arr.pop();

    println!("{:?}", arr);

    // Change value
    arr[0] = 20;

    // Get single value
    println!("Single value: {}", arr[0]);

    // Get Vector length
    println!("Vector length: {}", arr.len());

    // Vectors are stack allocated
    println!("Vector occupied: {}", mem::size_of_val(&arr));

    // Get slice from Vector
    let slice: &[i32] = &arr[1..3]; // means from 1 to 3-1
    println!("Slice: {:?}", slice);

    // Loop through vector
    for x in arr.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate
    for x in arr.iter_mut() {
        *x = *x * 2;
    }
    println!("Numbers: {:?}", arr);
}