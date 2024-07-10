


fn generic_function<T: std::fmt::Debug, U: std::fmt::Debug>(value1: T, value2: U) {
    println!("{:?} {:?}", value1, value2);
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // generic_function(42, "Hello");
    // generic_function("Hello");
    // generic_function(3.14);

    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };

    println!(" Interger points : ({}, {})", p.x, p.y);
    println!(" Float points : {:?}, {:?}", p2.x, p2.y);
}

