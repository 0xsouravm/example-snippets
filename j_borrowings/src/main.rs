fn main() {
    example4();
    example5();
    example6();
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

fn example4() {
    let s1 = String::from("hello");
    let k1 = String::from("world");
    let mut s2 = &s1;
    s2 = 50;
    println!("s2 is {}", s2);
    s2 = &k1;
    println!("s2 is {}", s2);
    *s2 = String::from("hello world");
}


fn example5() {
    let mut s1 = String::from("hello");
    let mut k1 = String::from("world");
    let s2 = &mut s1;
   s2.push_str(" world");
    println!("s2 is {}", s2);
    s2 = &mut k1;
}


fn example6() {
    let s1 = String::from("hello");
    let k1 = String::from("world");
    let mut s2 = &mut s1;
    println!("s2 is {}", s2);
    s2.push_str(" world");
    println!("s2 is {}", s2);
    *s2 = &mut k1;
}

// s = hello
// s = hello world

// x > "hello"

// &x > the location of x

// s > &x > 'hello'