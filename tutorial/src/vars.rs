pub fn run() {
    println!("Vars-----------------------------------------------------------------------");

    let name = "Mohamed";
    let mut age = 35;

    println!("My name is {}, and I am {}", name, age);

    age = 36;

    println!("Now I am {}", age);

    const ID: i32 = 001;

    println!("ID: {}", ID);

    let (my_name, my_age) = ("Mohamed", 35);
    println!("My name is {}, and I am {}", my_name, my_age);

    println!("Vars-----------------------------------------------------------------------\n");
}
