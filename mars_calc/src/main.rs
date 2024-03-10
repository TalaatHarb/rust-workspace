use std::io;
use std::io::Write;

mod calc;

fn main() {
    let mut quit: bool = false;

    print!("Enter weight on Earth: ");
    io::stdout().flush().unwrap(); // necessary because print! is buffered until you flush

    let mut input: String = String::new();
    while !quit {
        io::stdin().read_line(&mut input).unwrap(); // reads until <Enter> and appends to string (this is why clearing is important)

        match input.trim().parse::<f32>() {
            Ok(weight_on_earth) => {
                println!("Weight on Earth = {}", weight_on_earth);
                println!(
                    "Weight on Mars = {}",
                    calc::calculate_weight_on_mars(&weight_on_earth)
                );
                quit = true;
            }
            Err(err) => {
                eprintln!(
                    "Unable to parse input ({}) to float, {:?}\nPlease enter a valid float!",
                    input.trim(),
                    err 
                );
                input.clear(); // necessary to clear the contents before reading again into the string
            }
        }
    }
}

