pub fn run() {
    // printing
    println!("Print----------------------------------------------------------------------");

    // formatting
    println!("Place holder value {}", 1);

    // positional argument
    println!("{0} is an interesting {1}, I like {0}.", "Rust", "language");

    // named arguments
    println!("{name} likes to play {activity}.", name = "Mohamed", activity = "Chess");

    // traits
    println!("Binary\t: {:b},\nHex\t: {:x},\nOctal\t: {:o}", 10, 10, 10); // number format
    println!("{:?}", (12, true, "Hi")); // debug trait

    println!("Print----------------------------------------------------------------------\n");
}
