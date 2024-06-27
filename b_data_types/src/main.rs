fn main() {
    // i8_example();
    // i16_example();
    // i32_example();
    // i64_example();
    // i128_example();

    // u8_example();
    // u16_example();
    // u32_example();
    // u64_example();
    // u128_example();

    // boolean_example();

    // char_example();

    // tuple_example();
    array_example();
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

fn tuple_example() {
    // println!("Tuple Example");

    // let person: (char, i32, f32) = ('A', 24, 100.50);
    // println!("The value of person is: {:?}", person);

    // // Access elements by indices
    // let name = person.0;
    // let age = person.1;
    // let salary = person.2;

    // println!("The name of the person is: {}", name);
    // println!("The age of the person is: {}", age);
    // println!("The salary of the person is: {}\n", salary);

    // // Destructuring
    // println!("Destructuring");
    // let (name, age, salary) = person;
    // println!("The name of the person is: {}", name);
    // println!("The age of the person is: {}", age);
    // println!("The salary of the person is: {}\n", salary);

    // Mutable Tuple
    // let mut mutable_tuple = (1, 2, 3);
    // println!("The value of mutable_tuple is: {:?}", mutable_tuple);

    // mutable_tuple.0 = 445159;
    // println!("The mutated value of mutable_tuple is: {:?}", mutable_tuple);

    // let tuple = (1, 2, 3);
    // println!("The value of tuple is: {:?}", tuple);
}

fn array_example() {
    println!("Array Example");

    let arr_of_numbers = [1, 2, 3, 4, 5];
    println!("The value of arr_of_numbers is: {:?}", arr_of_numbers);

    // Access Elements
    let first_element = arr_of_numbers[0];
    let second_element = arr_of_numbers[1];
    let third_element = arr_of_numbers[2];

    println!("The first element of arr_of_numbers is: {}", first_element);
    println!("The second element of arr_of_numbers is: {}", second_element);
    println!("The third element of arr_of_numbers is: {}\n", third_element);

    // let some_decimal = 100.90_f32;
    // let some_integer = 100u8;
    // let some_large_integer = 100283472319857235i64;

    let arr_of_repeated_elements = [0; 5];
    let another_arr_of_repeated_elements: [i8; 15] = [0; 15];
    let another_another_arr_of_repeated_elements = [0u8; 15];

    println!("The value of arr_of_repeated_elements is: {:?}", arr_of_repeated_elements);
    println!("The value of another_arr_of_repeated_elements is: {:?}\n", another_arr_of_repeated_elements);
    println!("The value of another_another_arr_of_repeated_elements is: {:?}\n", another_another_arr_of_repeated_elements);
}