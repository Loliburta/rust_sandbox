// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;

// print::run();
// vars::run();
// types::run();
// strings::run();
// tuples::run();
// arrays::run();
// vectors::run();
// conditionals::run();
// loops::run();
// functions::run();
// pointer_ref::run();
// structs::run();
// enums::run();

mod enums;
fn main() {
    let mut s = String::from("abc");
    s = s.chars();
}

pub fn remove_char(s: &str) -> String {
    println!("s");
    return String::from(s);
}
