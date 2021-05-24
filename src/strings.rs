// Primitive str = Immutable fixed-length string somewhere in the memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {

    // Not mutable str
    let ab = "abc";

    // Mutable String
    let mut hello = String::from("hello what is up dog");

    // Get length
    println!("Length: {}", hello.len());

    // push is for Characters
    hello.push('!');

    // push_str is for Strings
    hello.push_str("!!!");

    // Capacity in bytes
    println!("capacity: {}", hello.capacity());

    // Check if empty string
    println!("Is empty {}", hello.is_empty());

    // Contains
    println!("contains 'he' {}", hello.contains("he"));

    // Replace
    println!("Replace: {}", hello.replace("!", "?"));

    // Loop through string by whitespace

    for word in hello.split(" ") {
        println!("{}", word)
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');

    // assertion testing
    assert_eq!(1, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}


/* Useful string methods
.trim() removes whitespace at the beginning and end 
*/