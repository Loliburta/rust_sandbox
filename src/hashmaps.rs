use std::collections::HashMap;
pub fn run() {
    //create new hashmap
    let mut marks = HashMap::new();

    //add values
    marks.insert("Rust programming", 10);
    marks.insert("Python", 20);

    //find a value in hashmap
    match marks.get("Rust programming") {
        //if found a value
        Some(mark) => {
            println!("found ya")
        }
        // Didn't find a value
        None => println!("value doesn't exist"),
    }

    // remove a value from hashmap
    marks.remove("Rust programming");

    //loop through a hashmap
    for (key, value) in &marks {
        println!("key: {key} value: {value}", key = key, value = value);
    }
}
