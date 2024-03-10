use std::env;

pub fn run() {
    println!("CLI------------------------------------------------------------------------");

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let vars: Vec<(String, String)> = env::vars().collect();
    println!("Vars: {:?}", vars);

    println!("CLI------------------------------------------------------------------------\n");
}
