pub fn run() {
    println!("Structs--------------------------------------------------------------------");
    let c1: ColorS = ColorS{red: 201, green: 0, blue: 0};
    println!("{} {} {}", c1.red, c1.green, c1.blue);

    let c2: ColorT = ColorT(202, 0, 0);
    println!("{} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Mary", "Doe");
    println!("{}", p.full_name());

    p.set_last_name("Wiliams");
    println!("{}", p.full_name());

    
    println!("Structs--------------------------------------------------------------------\n");
}

struct ColorS{
    red: u8,
    green: u8,
    blue:u8
}

struct ColorT(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person{
        return Person{first_name: first.to_string(), last_name: last.to_string()};
    }

    fn full_name(&self) -> String{
        return format!("{} {}", self.first_name, self.last_name);
    }

    fn set_last_name(& mut self, last: &str){
        self.last_name = last.to_string();
    }
}