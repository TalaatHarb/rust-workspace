pub fn run() {
    println!("Enums----------------------------------------------------------------------");

    move_player(Movement::Up);
    move_player(Movement::Down);
    move_player(Movement::Left);
    move_player(Movement::Right);

    println!("Enums----------------------------------------------------------------------\n");
}

enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_player(movement: Movement){
    match movement {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right")
    }
}
