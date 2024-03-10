pub fn run() {
    println!("Pointers-------------------------------------------------------------------");

    // primitive
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // non-primitive
    let vec1: Vec<i32> = vec![1, 2, 3];
    // let vec2: Vec<i32> = vec1; // move operation which renders vec1 unusable
    let vec2: &Vec<i32> = &vec1; // reference

    println!("Values: {:?}", (&vec1, vec2));

    println!("Pointers-------------------------------------------------------------------\n");
}
