enum Movement {
    // Variants
    Up,
    Down,
    Right,
    Left,
}
fn move_avatar(m: Movement) {
    // perform action depending on movement
    match m {
        Movement::Up => println!("moving up"),
        Movement::Down => println!("moving down"),
        Movement::Right => println!("moving right"),
        Movement::Left => println!("moving left"),
    }
}
pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
}
