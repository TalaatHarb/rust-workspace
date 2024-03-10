pub fn run() {
    println!("Types----------------------------------------------------------------------");

    println!("u8\t: Min: {}, Max: {}", std::u8::MIN, std::u8::MAX);
    println!("i8\t: Min: {}, Max: {}", std::i8::MIN, std::i8::MAX);
    println!("u16\t: Min: {}, Max: {}", std::u16::MIN, std::u16::MAX);
    println!("i16\t: Min: {}, Max: {}", std::i16::MIN, std::i16::MAX);
    println!("u32\t: Min: {}, Max: {}", std::u32::MIN, std::u32::MAX);
    println!("i32\t: Min: {}, Max: {}", std::i32::MIN, std::i32::MAX);
    println!("usize\t: Min: {}, Max: {}", std::usize::MIN, std::usize::MAX);
    println!("u64\t: Min: {}, Max: {}", std::u64::MIN, std::u64::MAX);
    println!("i64\t: Min: {}, Max: {}", std::i64::MIN, std::i64::MAX);
    println!("u128\t: Min: {}, Max: {}", std::u128::MIN, std::u128::MAX);
    println!("i128\t: Min: {}, Max: {}", std::i128::MIN, std::i128::MAX);
    println!("f32\t: Min: {}, Max: {}", std::f32::MIN, std::f32::MAX);
    println!("f64\t: Min: {}, Max: {}", std::f64::MIN, std::f64::MAX);

    let (x, y, z, is_active) = (23, 2.5, "3456", true);
    println!("{:?}", (x, y, z, is_active));

    let emoji: char = '\u{1F600}';

    println!("{}", emoji);

    println!("Types----------------------------------------------------------------------\n");
}
