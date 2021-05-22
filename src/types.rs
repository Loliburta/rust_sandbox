/*
Primitive Types
Integers: u8, i8, u16, u32, u64, i64, u128, i128, (number of bits)
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays
*/

// Rust is a statically typed language, compiler can infer types

pub fn run() {
    // Default i32
    let x = 1;

    // Default f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 449324823;

    // Find max size

    println!("Max i32: {Max}", Max = std::i32::MAX);
    println!("Max f64: {Max}", Max = std::f64::MAX);

    // Boolean
    let is_active = true;

    // Get Boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    println!("{:?}", (x, y, z, is_active, is_greater, a1))
}
