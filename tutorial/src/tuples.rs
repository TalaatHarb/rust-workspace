pub fn run() {
    println!("Tuples---------------------------------------------------------------------");
    // max 12 elements in a tuple
    let person: (&str, &str, u8) = ("Mohamed", "Egypt", 35);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    

    println!("Tuples---------------------------------------------------------------------\n");
}
