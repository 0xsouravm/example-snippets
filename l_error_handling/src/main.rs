


// enum Option<T> {
//     Some(T),
//     None,
// }


fn find_item(value: i32) -> Option<i32> {
    if value == 42 {
        Some(value)
    } else {
        None
    }
}



// fn main() {
//     let item = find_item(2);
//     match item {
//         Some(value) => println!("Found: {}", value),
//         None => println!("Not found"),
//     }

//     let item = find_item(42);
//     match item {
//         Some(value) => println!("Found: {}", value),
//         None => println!("Not found"),
//     }
// }


// fn main() {
//     let some_val = Some(42);

//     // let some_val: Option<i32> = None;

//     if some_val.is_some(){
//         let value = some_val.unwrap();
//         println!("Found: {}", value);
//     }else{
//         println!("Not found");
//     }
    
//     // println!("Found: {}", value);
// }

// some(value) -> value

// none -> panic


// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 6 /2 =3

// 6/0 = error

fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("Denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}



fn main() {
    let result = 10/3;
    println!("Result: {:?}", result);
    // match result {
    //     Ok(value) => println!("Result: {}", value),
    //     Err(error) => println!("Error: {}", error),
    // }
}


// result handles both success and error return types
// it improves error 



// 10 / 3

// 3 x 3 = 9

// 10 - 9 = 1