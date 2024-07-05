use std::arch::aarch64::vraddhn_high_s64;

fn main() {
    // println!("Hello, world!");
}

// Enums: Allow you to define a type by enumerating its possible variants / a set of named values that are possible for the type.

enum HeterogeneousList {
    Integer(i32),
    Float(f32),
    Text(String),
    RandomTypes(bool, char, i32, i8, f64)
}

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

enum Shapes {
    Circle(u8),
    Rectangle(u8, u8),
    Square(u8),
    EquilateralTriangle(u8),
}

fn area(shape: Shapes) -> f32 {
    match shape {
        Shapes::Circle(radius) => (3 * radius * radius) as f32,
        Shapes::Rectangle(length, breadth) => (length * breadth) as f32,
        Shapes::Square(side) => (side * side) as f32,
        Shapes::EquilateralTriangle(side) => (side * side) as f32,
    }
}

impl Shapes {
    // method to find the area
    fn area(&self) -> f32 {
        match self {
            Shapes::Circle(radius) => (3 * radius * radius) as f32,
            Shapes::Rectangle(length, breadth) => (length * breadth) as f32,
            Shapes::Square(side) => (side * side) as f32,
            Shapes::EquilateralTriangle(side) => (side * side) as f32,
        }
    }

    // Associated function to create a new circle from the given radius
    fn return_circle(radius: u8) -> Self {
        Shapes::Circle(radius)
    }
}

fn find_area_of_shape() {
    let circle = Shapes::Circle(10);
    let area_of_circle = circle.area();


    let new_circle = Shapes::return_circle(10);
    let new_circle_area = new_circle.area();

    let area_of_circle = area(circle);
}

enum GameMoves {
    Fireball,
    IcicleShower,
    WindSlash,
    EarthWall,
    BlindingLight // Variant
}

fn game_moves_example() {
    let fire_move = GameMoves::Fireball;
    let wind_move = GameMoves::WindSlash;
    let earth_move = GameMoves::EarthWall;
    let light_move = GameMoves::BlindingLight;
    let ice_move = GameMoves::IcicleShower;

    let another_fire_move = GameMoves::Fireball;

    // let some_integer = 100;

    let some_random_move = GameMoves::WindSlash;
    // match some_integer {
    //     100 => println!("It is 100"),
    //     _ => println!("It is not 100")
    // }

    match fire_move {
        GameMoves::Fireball => println!("Fireball"),
        _ => println!("Not Fireball")
    }

    match some_random_move {
        GameMoves::Fireball => println!("Fireball"),
        GameMoves::IcicleShower => println!("IcicleShower"),
        GameMoves::WindSlash => println!("WindSlash"),
        GameMoves::EarthWall => println!("EarthWall"),
        GameMoves::BlindingLight => println!("BlindingLight"),
    }
}

// enum SomeEnum {
//     Variant1: i32
//     Variant2: u32,
// }

// struct Student {
//     var1: i32,
//     var2: u32
// }

enum AnotherEnum {
    VariantWithTypes { var1: i32 }, // Struct Variant
    // VariantRegular(i32, u32), // Tuple Variant
    // Rectangle { length: u32, breadth: u32 },
    // Circle { radius: u32 },
    // CircleWithDiameter { diameter: u32 },
}

fn struct_variant_example() {
    let variant_with_types = AnotherEnum::VariantWithTypes { var1: 10 };
    
    match variant_with_types {
        AnotherEnum::VariantWithTypes { var1 } => println!("VariantWithTypes: {}", var1),
    }
}