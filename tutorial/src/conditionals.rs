pub fn run() {
    println!("Conditionals---------------------------------------------------------------");

    let age: u8 = 35;
    let check_id: bool = false;

    if age >= 18 && check_id {
        println!("You can drive");
    } else if age < 18 && check_id {
        println!("You can not drive");
    } else {
        println!("Show me your ID");
    }

    println!("Conditionals---------------------------------------------------------------\n");
}
