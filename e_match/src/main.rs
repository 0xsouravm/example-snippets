// Match is a control flow statement that allows us to compare a value against
// a series of patterns and then execute code based on which pattern matches.

// Exhaustiveness

fn main() {
    // basic_match();
    // match_range();
    // match_with_guard();
    match_multiple();
}

fn basic_match() {
    let a = 100;
    let b = true;

    match b {
        true => println!("b is true"),
        false => println!("b is false"),
    }

    match a {
        1 => println!("a is 1"),
        2 => println!("a is 2"),
        3 => println!("a is 3"),
        4 => println!("a is 4"),
        5 => println!("a is 5"),
        100 => println!("a is 100"),

        // Default pattern/case
        _ => println!("a is something else"),
    }
}

fn match_range() {
    let a = 6;

    let b = false;

    let f = 80.23;

    match a {
        1..=12 => println!("a is between 1 and 12"),
        _ => println!("a is something else"),
    }
}

fn match_tuple() {
    let tup = (1, 2, 3, 4);

    match tup {
        // (_, _, _, _,) => println!("Tuple is 1, 2, 3, 4"),
        (1, .., 3) => println!("Last element is 4"),
        (1, 2, 3, 4) => println!("Tuple is 1, 2, 3, 4"),
        (1, ..) => println!("First element is 1"),
        (.., 3) => println!("Last element is 4"),
        (_, 2, ..) => println!("Second element is 2"),
        (_, _, 3, ..) => println!("Third element is 3"),
        (.., 4) => println!("Fourth element is 4"),
        // (_, _, _, 4) => println!("Fourth element is 4"),
        (x, y, z, a) => println!("{}, {}, {}, {}", x, y, z, a),
        // (1, _, _, _) => println!("First element is 1"),
        // (_, 2, _, _) => println!("Second element is 2"),
        _ => println!("Tuple is something else"),
    }
}

fn match_with_code_block() {
    let a = 100;
    match a {
        100 => {
            println!("a is 100");
            println!("a is 100");
            println!("a is 100");
        }

        _ => println!("a is something else"),
    }
}

// match with guard

// Guard - Additional condition that allows for extra filteration
fn match_with_guard() {
    let a = 200;
    // let b = 1..=100;
    // let b = 1..100;
    match a {
        0..=200 if a % 2 == 0 => {
            println!("a is between 0 and 200 and is even");
        },
        _ => println!("a is something else"),
    }
}

// a = 3 | 8 | 203;

// a % 2 == 0/1

fn match_multiple() {
    // let a = 3;
    // let a = 8;
    let a = 1234567;
    match a {
        3 | 8 | 203 => {println!("a is 3, 8, or 203")},
        _ => println!("a is something else"),
        // Wildcard Pattern
    }
}

// (1, 2, 4, 5, 6, 7, 8, 9, 10);
// (1, 2, 4, 5, 1, 2, 3, 4, 6, 7, 8, 9, 10);
// (1, 2, 6, 10);

// (..)
