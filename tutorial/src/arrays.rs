
pub fn run() {
    println!("Arrays---------------------------------------------------------------------");

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("First: {}", numbers[0]);

    // Re-assign
    numbers[1] = 0;
    println!("Second: {}", numbers[1]);

    // memory
    println!("Length: {}", numbers.len());
    println!("Memory: {}", std::mem::size_of_val(&numbers)); // numbers.len() * size_of_val(numbers[0])

    // slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("Slice Memory: {}", std::mem::size_of_val(&slice)); // always the same 2*uszie

    println!("Arrays---------------------------------------------------------------------\n");
}
