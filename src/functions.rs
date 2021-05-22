// Functions - just like in any other language
pub fn run() {
    greeting("Hello", "Loliburta");

    // Print function
    println!("{:?}", add(10, 20));

    // Bind function values to Variables
    let get_sum = add(1220, 210);
    println!("{:?}", get_sum);

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}
fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2; // or just n1 + n2 without return and semicolon
}
