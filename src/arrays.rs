// Arrays - Homogenous elements
// Once declared - Fixed in size

use std::mem;

pub fn run() {

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // Change value
    arr[0] = 20;

    // Get single value
    println!("Single value: {}", arr[0]);

    // Get array length
    println!("Array length: {}", arr.len());

    // Arrays are stack allocated
    println!("Array occupied: {}", mem::size_of_val(&arr));

    // Get slice from array
    let slice: &[i32] = &arr[1..3]; // means from 1 to 3-1
    println!("Slice: {:?}", slice);
}