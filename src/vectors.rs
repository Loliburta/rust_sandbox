// Vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign values
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);

    // Remove last value
    numbers.pop();

    // Get vector length
    println!("{}", numbers.len());

    // Vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // Loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // Loop and mutate values (kind of like a map in js)
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("{:?}", numbers);
}