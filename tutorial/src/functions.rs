pub fn run() {
    println!("Functions------------------------------------------------------------------");

    // calls
    greeting("Hello", "Mohamed");
    println!("5 + 3 = {}", add(5, 3));

    // closures
    let n3 = 9;
    let add_nums = |n1: i32, n2: i32| n1+n2+n3;
    println!("3 + 3 + 9 = {}", add_nums(3, 3));

    println!("Functions------------------------------------------------------------------\n");
}

// non-mutable inputs
fn greeting(greet: &str, name: &str){
    println!("{} {}", greet, name);
}

// output
fn add(n1: i32, n2: i32) -> i32{
    return n1 + n2;
}