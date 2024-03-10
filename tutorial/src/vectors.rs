pub fn run() {
    println!("Vectors--------------------------------------------------------------------");
    
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("First: {}", numbers[0]);

    // Re-assign
    numbers[1] = 0;
    println!("Second: {}", numbers[1]);

    // push/pop
    numbers.push(6);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);

    // memory
    println!("Length: {}", numbers.len());
    println!("Capacity: {}", numbers.capacity());
    println!("Memory: {}", std::mem::size_of_val(&numbers)); // numbers.len() * size_of_val(numbers[0]) + size_of(u32)

    // slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("Slice Memory: {}", std::mem::size_of_val(&slice)); // always the same 2* size_of(uszie)

    // loop through vector
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("{:?}", numbers);
    

    println!("Vectors--------------------------------------------------------------------\n");
}
