// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "loliburta";
    let mut age = 19;
    println!("My name is {Name} and was {Age}", Name = name, Age = age);
    age = 20;
    println!("My name is {Name} and I'm {Age}", Name = name, Age = age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {ID}", ID = ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Loliburta", 20);
    println!("{My_name}, {My_age}", My_name = my_name, My_age = my_age);

}
