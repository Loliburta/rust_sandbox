// Any other if else in any language
pub fn run() {
    let age: u8 = 18;
    let check_id = true;
    let knows_person_of_age = true;
    //if else
    if age >= 21 && check_id || knows_person_of_age  {
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id {
        println!("Bartender: sorry you're not allowed to drink");
    }else{
        println!("Bartender: I'll need to see your ID")
    }

    // Shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("{}", is_of_age);

}