fn main() {
    // println!("Hello, world!");
}

// Enums: Allow you to define a type by enumerating its possible variants / a set of named values that are possible for the type.

enum Direction {
    North,
    East,
    South,
    West
}

enum DirectionWithValues {
    North(i32),
    East,
    West,
    South
}

fn choose_direction_with_values() {
    let north: DirectionWithValues = DirectionWithValues::North(10);

    match north {
        DirectionWithValues::North(_) => println!("It is North"),
        _ => println!("It is not North")
    }

    match north {
        DirectionWithValues::North(step_count) => {
            if step_count == 10 {
                println!("It is North with step count: {}", step_count)
            }
        },
        _ => println!("It is not North")
    
    }
}

fn choose_direction() {
    // TypeName::Variant
    let east: Direction = Direction::East;
    let north = Direction::North;

    match east {
        Direction::East => println!("East"),
        _ => println!("Not East")
    }

    match north {
        Direction::North => println!("North"),
        _ => println!("Not North")
    }

    match north {
        Direction::East => println!("East"),
        Direction::North => println!("North"),
        Direction::South => println!("South"),
        Direction::West => println!("West"),
    }
}