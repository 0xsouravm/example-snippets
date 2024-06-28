fn main() {
    // example1();
    // example2();
    // example4();
    // example5();
    // example6();
    // example7();
    // example8();
    example9();
}

fn example1() {
    // Does not work based on conditionals
    let mut count = 1;

    loop {
        count += 1;
        print!("Hello, world!");
        if count == 5 {
            break;
        }
    }
}

fn example3() {
    // Does not work based on conditionals
    let mut count = 1;
    let val_to_check = true;
    while val_to_check {
        count += 1;
        print!("Hello, world!");
        if count == 5 {
            break;
        }
    }
}

fn example2() {
    let mut num = 3;

    while num != 10 {
        println!("{}!", num);
        num += 1;
    }

    println!("Loop ended");
}

// count = 0
// count < 30
// count++

fn example4() {
    for num in 0..30 {
        println!("{}!", num);
    }
}

fn example5() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        if *element < 50 {
            println!("{}", element);
        }
    }
}

fn example6() {
    // let arr = [10, 20, 30, 40, 50];

    for num in 0..30 {
        println!("{}!", num);
        if num == 20 {
            break;
        }
    }
}

// for
// while
// loop

fn example7() {
    for num in 4..30 {
        if num % 2 == 0 {
            continue;
        } else {
            break;
        }
        println!("{}", num);
    }
}

fn example8() {
    for i in 1..5 {
        for j in 1..5 {
            println!("i: {}, j: {}", i, j);
        }
    }
}

fn example9() {
    let number = 10;
    let result = if number > 5 {
        "Greater";
    } else {
        "Smaller or Equal"
    };
    println!("{}", result);
}

// (10, 35.7, "hello")

// 0 - 35.7
