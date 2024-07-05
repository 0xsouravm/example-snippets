
fn main(){
    // example1();
    // example2();
    // example3();
    example4();
    // example5();
}

fn example1() {
   let x = 10;// x owns the value 10
   println!("x is {}", x);// We can print x
}

fn example2() {
    let x = 10;// x owns the value 10
    let y =x; // y now owns the value 10
   println!("x is {}", x);// We can print x
    println!("y is {}", y);// We can print y
}

fn example3(){
    let x = "Hello".to_string();// x owns the value "Hello"
    let y = x;
    // println!("x is {}", x);
    println!("y is {}", y);
}

fn give_ownership(x: String) {
    println!("x is {}", x);
}

fn example4() {
    let x = String::from("Hello"); // x is owner of 5
    println!("x is {}", x); // print x 
    let y = x.clone(); // cloning x to y    
    give_ownership(y.clone()); // passing y to the function
    println!("x is {}", x); // printing x again
    println!("y is {}", y); // printing y
}


// fn example5(){
//     let x = "Hello".to_string();// x owns the value "Hello"
//     let y = x.clone();
//     println!("x is {}", x);
//     println!("y is {}", y);
//     //ssome

// }

// raj -  book
// book 1= book.clone()
// yuv - book 1

// raj - book

