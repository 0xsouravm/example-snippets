fn main() {
    // example1();
    // example2();
    // example3();
    // example4();
    // example5();
    example6();
    example7();
    example8();
    weather();
}

fn example1() {
    if true {
        println!("If statement worked");
    }

    if false {
        println!("If statement worked");
    }
}

fn example2() {
    let age = 20;

    if age < 30 {
        println!("exmple2: You are young");
    }
}

fn example3() {
    let age = 40;

    if age < 30 {
        println!("You are young");
    }
}

fn example4() {
    let age = 40;

    if age > 20 {
        println!("You are not young");
    }
}

fn example5() {
    let age = 30;

    if age == 20 {
        println!("You are 20");
    } else {
        println!("You are not young");
    }
}

fn example6() {
    let age = 45;

    if age < 20 {
        println!("you are less than 20"); //0-19
    } else if age < 30 {
        println!("you are less than 30"); // 20-29
    } else if age < 40 {
        println!("you are less than 40"); // 30 and above
    }
}

fn example7() {
    let mark = 90;
    let grade = if mark > 90 { "A" } else { "B" };
    println!("Grade: {}", grade);
}

fn example8() {
    let age = 15;
    if age > 20 {
        if age < 30 {
            println!("you are less than 30");
        } else {
            println!("you are 30 and above");
        }
    } else {
        println!("you are less than 20");
    }
}

fn weather() {

    let is_raining = true;
    let is_windy = true;

    if is_raining && is_windy {
        println!("It's a thunderstorm")
    } else if is_raining || is_windy{
        println!("It's a good weather")

    }else{
        println!("It's a sunny day")
    }
}

// how if works
// how to write if statement
// how to handle false cases in if statement
// how to write multiple if else 
// how to write nested if statements

// age < 20
// 0-19
// 20-29

// if true {

// } else {

// }

// true false
// if true{
//     execute this code
// }
// else{
//     execute this code

// }

// Operators

// Arithmetic operations (+, -, *, /, %)
// Comparison operations (==, !=, >, <, >=, <=) bool true/ false
// Logical operations (&&, ||, !) bool true/ false
// Bitwise operations (&, |, ^, !, <<, >>)
