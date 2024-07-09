fn main() {
    // example4();
    // example5();
    // example6();
    // example8();
    rule2();
}

fn example1() {
    let x = String::from("Hello"); // x owns the value 10
    let y = &x;
    println!("y is {}", y);
    let len = cal_length(&x); // x is moved to the function
    println!("Length of x is {}", len);
    println!("x is {}", x); // We can print x
}

fn cal_length(s: &String) -> usize {
    println!("s is {}", s);
    s.len()
}

fn example2() {
    let mut x = String::from("Hello"); // x owns the value 10
                                       // let y = &x;
                                       // println!("y is {}", y);
    test_mutable_ref(&mut x); // x is moved to the function
                              // println!("Length of x is {}", len);
    println!("x is {}", x); // We can print x
}

fn test_mutable_ref(s: &mut String) {
    println!("s is {}", s);
    s.push_str(" world");
    println!("s is {}", s);
}

fn example3() {
    let mut x = 10; // x owns the value 10
                    // let y = &x;
                    // println!("y is {}", y);
    double(&mut x); // x is moved to the function
                    // println!("Length of x is {}", len);
    println!("x is {}", x); // We can print x
}

fn double(s: &mut i32) {
    println!("s is {}", s);
    *s *= 2;
    println!("s is {}", s);
}

// fn example4() {
//     let s1 = String::from("hello");
//     let k1 = String::from("world");
//     let mut s2 = &s1;
//     s2 = &k1;
//     println!("s2 is {}", s2);
//     s2 = &k1;
//     println!("s2 is {}", s2);
//     *s2 = String::from("hello world");
// }

// fn example5() {
//     let mut s1 = String::from("hello");
//     let mut k1 = String::from("world");
//     let s2 = &mut s1;
//    s2.push_str(" world");
//     println!("s2 is {}", s2);
//     s2 = &mut k1;
// }

// fn example6() {
//     let mut s1 = String::from("hello");
//     let mut k1 = String::from("world");
//     let mut s2 = &mut s1;
//     println!("s2 is {}", s2);
//     // s2.push_str(" world");
//     println!("s2 is {}", s2);
//     *s2 = &mut k1;
// }

// s = hello
// s = hello world

// x > "hello"

// &x > the location of x

// s > &x > 'hello'

fn example7() {
    let mut num = 10; // 10 is owned by num
    let num_ref = &mut num; //num_ref is a mutable reference to num
    *num_ref += 10;
    println!("num is {}", num);
}

struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn example8() {
    let mut x = 5;

    let y = &mut x;

    test2(y);

    let z = &mut x;
    // test1(y, z);

    println!("x is {}", x);

    println!("x is {}", x);
}

fn test1(s: &mut i32, k: &mut i32) {
    *s += 1;
    *k += 1;
}

fn test2(s: &mut i32) {
    *s += 2;
}

// num -> 10

// num_ref -> &num -> 10

// *num_ref -> 20

// You can have multiple immutable references to a value
fn rule1() {
    let s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("r1 is {}", r1);
    println!("r2 is {}", r2);
}

// You can have only one mutable reference to a value
fn rule2() {
    let mut s1 = String::from("hello");

    let r2 = &s1;
    let r1 = &mut s1;

    // println!("r2 is {}", r2);
    // println!("r1 is {}", r1);

    // let r3 = &mut s1;

    // println!("r2 is {}", r1);
}

// lib - book

// raj  book

// raj returns
// yuv
