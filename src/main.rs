/*
mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
*/

/*
print::run();
vars::run();
types::run();
strings::run();
tuples::run();
arrays::run();
vectors::run();
conditionals::run();
loops::run();
functions::run();
pointer_ref::run();
structs::run();
enums::run();
*/

// Projects
mod projects;
use projects::guessing_game;

/*
How to import functions from subFolders?
1. Create a file with name of subfolder
2. Write there every file you're going to use in
3. Import this file to main using "mod" before it ex. mod projects;
4. use word "use" name_of_file + :: + name_of_function ex. use projects::guessing_game;
*/

/*
How to add dependencies?
1. Go to Cargo.toml and type name = "version" ex. rand = "0.8.3"
2. cargo build
(3. to update dependencies use cargo update)
*/


fn main() {
    guessing_game::run();
}
