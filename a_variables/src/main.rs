// const THREE_MINUTES_IN_SECONDS: u32 = 180;
const THREE_MINUTES_IN_SECONDS: u32 = 3 * 60; // Global Scope

fn main() {
    immutable_variable_example();
    mutable_variable_example();
    constant_example();
    shadowing_example();
    scope_example();
    type_annotation_example();
    mutable_vs_shadowing_example();
    destructuring_example();
    destructuring_with_underscore_example();
    advanced_destructuring_example();
    very_advanced_destructuring_example();
    const LOCAL_THREE_MINUTES_IN_SECONDS: u32 = 3 * 60; // Local Scope

}

fn immutable_variable_example() {
    println!("Immutable Variable Example");
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    println!();
}

fn mutable_variable_example() {
    println!("Mutable Variable Example");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);
    println!();
}

fn constant_example() {
    println!("Constant Example");
    println!("The value of CONSTANT is: {}", THREE_MINUTES_IN_SECONDS);
    // CONSTANT = 6;
    println!();
}

fn shadowing_example() {
    println!("Shadowing Example");
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = true;
    println!("The value of x is: {}", x);
    println!();
}

fn scope_example() {
    println!("Scope Example");
    //---------------------------------------------- OUTER SCOPE
        let x = 100;
        {
            //------------------------------------------ INNER SCOPE
            let y = 200;
            println!("The value of y is: {}", y);
            println!("The value of x in inner scope is: {}", x);
            //------------------------------------------ INNER SCOPE
        }
        println!("The value of x in outer scope is: {}", x);
    //---------------------------------------------- OUTER SCOPE
}

fn type_annotation_example() {
    println!("Type Annotation Example");
    let x: i32 = 5;
    println!("The value of i32 x is: {}", x);
    
    let x: u8 = 5;
    println!("The value of u8 x is: {}", x);
    println!();
}

fn mutable_vs_shadowing_example() {
    println!("Mutable vs Shadowing Example");
    // Can change the value of the var but not the type
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    // Can change both value and type
    let x = false;
    println!("The value of x is: {}", x);
    println!();
}

fn destructuring_example() {
    println!("Destructuring Example");
    let (x, y) = (1, 2);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!();
}

fn destructuring_with_underscore_example() {
    println!("Destructuring with Underscore Example");
    // Not Recommended. Why?
    let (x, _) = (1, 2);
    println!("The value of x is: {}", x);
    println!();
}

fn advanced_destructuring_example() {
    println!("Advanced Destructuring Example");
    let (first, ..) = (1, 2, 3);
    println!("The value of first is: {}", first);

    let (.., last) = (1, 2, 3);
    println!("The value of last is: {}", last);

    // Incorrect Destructuring
    // let (.., mid, ..) = (1, 2, 3);
    println!();
}

fn very_advanced_destructuring_example() {
    println!("Very Advanced Destructuring Example");
    struct Point {
        x: i32,
        y: i32,
    };

    let point = Point { x: 1, y: 2 };
    let Point { x, y } = point;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // Incorrect Destructuring
    // let Point { a, b } = point;
    println!();
}