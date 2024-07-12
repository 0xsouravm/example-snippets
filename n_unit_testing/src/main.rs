

// unit testing

// unit testing stands for testing each and every component of your software

// easy and early debugging, makes refactoring easy

// rust has integrated unit tests, which makes the unit testing easy and complete at program level 

// unit tests are done for each and every success and failure case

// division - 6 / 3 = 2
// 6 / 0 = error

// error failure case
// 6/3 2 
// 6/2 2
mod test;

fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
