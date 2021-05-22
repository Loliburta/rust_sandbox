pub fn run() {
    println!("Hello from print.rs");
    // Basic formatting
    println!("{} from {}", "hello", "matt");

    // Positional arguments
    println!("{0} from {1} {1}", "hello", "matt");

    // Named arguments
    println!(
        "{name} is learning {language}",
        name = "matt",
        language = "rust"
    );

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
