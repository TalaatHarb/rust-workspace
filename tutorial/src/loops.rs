pub fn run() {
    println!("Loops----------------------------------------------------------------------");

    let mut count:u32 = 0;

    // infinite
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 20 {
            break;
        }
    }

    // while (FizzBuzz)
    count = 1;
    while count <= 100{
        if count % 15 == 0{
            println!("FizzBuzz");
        }else if count % 3 == 0{
            println!("Fizz");
        }else if count % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", count);
        }
        count += 1;
    }

    // for range
    for x in 0..100{
        if x % 15 == 0{
            println!("FizzBuzz");
        }else if x % 3 == 0{
            println!("Fizz");
        }else if x % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", x);
        }
    }

    println!("Loops----------------------------------------------------------------------\n");
}
