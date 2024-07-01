fn main() {
    // normal_function();
    // fn_with_parameters(12, 36);
    let a = add_ten_to_twenty();
    let b = add(30, 40);
    println!("b: {}", b);
    return_with_if_else();
    let nothing = normal_function();
}

fn normal_function() -> () {
    println!("Normal Function");
}

fn another_normal_function() {
    println!("Normal Function");
}

// Parameters = a and b 
// Arguments = 12 and 36
fn fn_with_parameters(a: i32, b: i32) {
    println!("Function with Parameters");
    println!("a: {}", a);
    println!("b: {}", b);
}

fn add_ten_to_twenty() -> i32 {
    10 + 20
}

fn add(a: i32, b: i32) -> i32{
    // let c = a + b;
    // c
    // return c;
    a + b
    // return a + b;
}

fn return_with_if_else() -> i32 {
    let a = 10;
    if a % 2 == 0 {
        return 10;
    } else if a == 20 {
        return 0;
    } else {
        return 1;
    }
}

// fn something() {
//     let a = if 100 > 200 {
//         100;
//     } else {
//         200;
//     }
// }


// Arguments - Concrete values of the parameters that are passed to a function.
// Parameters - Variables in the signature of the function that tell you the type they expect and to which you can pass the concrete values.