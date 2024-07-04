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