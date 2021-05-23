// Arrays are not resizable

use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value 
    numbers[2] = 20;


    //Get single value
    println!("First value {}", numbers[0]);

    // Get array length
    println!("Array length {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice)
}
