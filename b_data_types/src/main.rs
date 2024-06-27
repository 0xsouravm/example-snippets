fn main() {
    i8_example();
    i16_example();
    i32_example();
    i64_example();
    i128_example();

    u8_example();
    u16_example();
    u32_example();
    u64_example();
    u128_example();

    boolean_example();

    char_example();
}

fn i8_example() {
    println!("i8 Example");
    let x: i8 = 127;
    println!("The value of x is: {}\n", x);

    let neg_x: i8 = -128;
    println!("The value of neg x is: {}\n", neg_x);
}

fn i16_example() {
    println!("i16 Example");
    let x: i16 = 32767;
    println!("The value of x is: {}\n", x);

    let neg_x: i16 = -32768;
    println!("The value of neg x is: {}\n", neg_x);
}

fn i32_example() {
    println!("i32 Example");
    let x: i32 = 2147483647;
    println!("The value of x is: {}\n", x);

    let neg_x: i32 = -2147483648;
    println!("The value of neg x is: {}\n", neg_x);
}

fn i64_example() {
    println!("i64 Example");
    let x: i64 = 9223372036854775807;
    println!("The value of x is: {}\n", x);

    let neg_x: i64 = -9223372036854775808;
    println!("The value of neg x is: {}\n", neg_x);
}

fn i128_example() {
    println!("i128 Example");
    let x: i128 = 170141183460469231731687303715884105727;
    println!("The value of x is: {}\n", x);

    let neg_x: i128 = -170141183460469231731687303715884105728;
    println!("The value of neg x is: {}\n", neg_x);
}

fn u8_example() {
    println!("u8 Example");
    let x: u8 = 255;
    println!("The value of x is: {}\n", x);
}

fn u16_example() {
    println!("u16 Example");
    let x: u16 = 65535;
    println!("The value of x is: {}\n", x);
}

fn u32_example() {
    println!("u32 Example");
    let x: u32 = 4294967295;
    println!("The value of x is: {}\n", x);
}

fn u64_example() {
    println!("u64 Example");
    let x: u64 = 18446744073709551615;
    println!("The value of x is: {}\n", x);
}

fn u128_example() {
    println!("u128 Example");
    let x: u128 = 340282366920938463463374607431768211455;
    println!("The value of x is: {}\n", x);
}

fn boolean_example() {
    println!("Boolean Example");
    let x: bool = true;
    println!("The value of x is: {}\n", x);

    let y: bool = false;
    println!("The value of y is: {}\n", y);
}

fn char_example() {
    println!("Char Example");
    let x: char = 'A';
    println!("The value of x is: {}\n", x);

    let y: char = '😻';
    println!("The value of y is: {}\n", y);

    let z: char = '汉';
    println!("The value of z is: {}\n", z);
}