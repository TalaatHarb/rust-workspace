pub fn run() {
    println!("Strings--------------------------------------------------------------------");

    let hello_prim: &str = "Hello";
    let mut hello: String = String::from(hello_prim);

    println!("{}, {}", hello_prim, hello);

    // length
    println!("{:?}", (hello_prim.len(), hello.len()));

    // push
    hello.push(',');
    hello.push_str(" World!");

    println!("{}", hello);

    // capacity / search
    println!("Capacity: {}, is_empty: {}, contains(World): {}", hello.capacity(), hello.is_empty(), hello.contains("World"));

    // replace
    println!("{}", hello.replace("World", "There")); // functional style replace
    println!("{}", hello);

    // split
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // assert
    let mut s: String = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


    println!("Strings--------------------------------------------------------------------\n");
}
