// Traits: allow types to implement certain methods that extend their functionality
// To Restrict

use std::fmt::{Debug, Display};

struct Car {
    model: String,
    year: u32,
}

struct Bike {
    model: String,
    year: u32,
}

struct Truck {
    model: String,
    year: u32,
}

enum VehicleType {
    Car,
    Bike,
    Truck,
}

impl Vehicle for VehicleType {
    fn drive(&self) {
        match self {
            VehicleType::Car => println!("Car Vroom Vroom"),
            VehicleType::Bike => println!("Bike Vroom Vroom"),
            VehicleType::Truck => println!("Truck Vroom Vroom"),
        }
    }

    fn honk_horn(&self) {
        match self {
            VehicleType::Car => println!("Car Beep Beep"),
            VehicleType::Bike => println!("Bike Beep Beep"),
            VehicleType::Truck => println!("Truck Beep Beep"),
        }
    }

    fn indicate_turn(&self) {
        match self {
            VehicleType::Car => println!("Car Flashing Turn Signal"),
            VehicleType::Bike => println!("Bike Flashing Turn Signal"),
            VehicleType::Truck => println!("Truck Flashing Turn Signal"),
        }
    }

    fn type_of_vehicle() {
        println!("Vehicle");
    }

}
// impl { 
    // Associated Functions -  First param is not self
    // Methods - First param is self
// }

// Drive
// Honk Horn
// Indicate Turn

trait VehicleDefined {
    // Default implementations of methods
    fn drive(&self) {
        println!("Vroom Vroom");
    }

    fn honk_horn(&self) {
        println!("Beep Beep");
    }

    fn indicate_turn(&self) {
        println!("Flashing Turn Signal");
    }
}

trait Vehicle {
    fn drive(&self); // meth
    fn honk_horn(&self) { // meth
        println!("Beep Beep");
    }
    fn indicate_turn(&self); // meth
    fn type_of_vehicle(); // associated function
}

impl Vehicle for Car {
    // Implement all functions
    // Implement Some - Make sure to implement all functions that are not default

    fn drive(&self) {
        println!("Car Vroom Vroom");
    }

    // You can reimplement a default method
    fn honk_horn(&self) {
        println!("Car Beep Beep");
    }
    
    fn indicate_turn(&self) {
        println!("Car Flashing Turn Signal");
    }

    fn type_of_vehicle() {
        println!("Car");
    }
}

impl Vehicle for Bike {
    fn drive(&self) {
        println!("Bike Vroom Vroom");
    }

    fn indicate_turn(&self) {
        println!("Bike Flashing Turn Signal");
    }

    fn type_of_vehicle() {
        println!("Bike");
    }
}

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck Vroom Vroom");
    }

    fn indicate_turn(&self) {
        println!("Truck Flashing Turn Signal");
    }

    fn type_of_vehicle() {
        println!("Truck");
    }
}

// Reimplement if needed
// impl VehicleDefined for Car {}
// impl VehicleDefined for Bike {}
// impl VehicleDefined for Truck {}

trait CarType {
    fn car_type();
}

impl CarType for Car {
    fn car_type() {
        println!("Sedan");
    }
}

fn main() {
    let car_var = Car {
        model: String::from("Toyota"),
        year: 2021,
    };

    let bike_var = Bike {
        model: String::from("Yamaha"),
        year: 2021,
    };

    let truck_var = Truck {
        model: String::from("Mercedes"),
        year: 2021,
    };

    // car - instance of Car
    // Car -  the type 'Car'

    car_var.drive();
    car_var.honk_horn();
    car_var.indicate_turn();
    Car::type_of_vehicle();
    Car::car_type();

    bike_var.drive();
    bike_var.honk_horn();
    bike_var.indicate_turn();
    Bike::type_of_vehicle();

    truck_var.drive();
    truck_var.honk_horn();
    truck_var.indicate_turn();
    Truck::type_of_vehicle();
}

// struct ABC;

// impl ABC {
//     fn some_associated_fn() {
//         println!("Some Associated Function");
//     }
// }

// trait SomeTrait {
//     fn some_fn(&self);
// }

// impl SomeTrait for ABC {
//     fn some_fn(&self) {
//         println!("Some Function");
//     }
// }

// struct Student {
//     name: String
// }

// impl Student {
//     fn new(name: String) -> Student {
//         Student {
//             name
//         }
//     }

//     fn print_name(&self) {
//         println!("{}", self.name);
//     }

//     fn random() {
//         println!("Random");
//     }
// }

// fn abc() {
//     let student = Student::new(String::from("John"));
//     student.print_name();
// }

// Trait Bounds - Bounds generic types to implement certain traits to be allowed use.

// Debug is a trait
// All types that implement the Debug trait can be used with "Debug Print".

// Issue: The generic function is printing values using the "Debug Print". 
// But not all types can be printed using the Debug Print.
// So the compiler is worried that this function might receive a type that cannot be printed using the Debug Print.

// Limit this function to only accept types that implement the "Debug" trait.

// fn generic_function<T: std::fmt::Debug>(value1: T) {
//     println!("{:?}", value1); // It is asking for a trait bound
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn test_trait_bounds() {
//     generic_function(34);
//     generic_function(true);
//     generic_function(60.45);
// }

// office {
//     Boss - Lounge
//     Manager - Lounge
//     Employee - No Lounge
// }

// fn access_lounge<T: Manager>(person: T){
//     println!("Welcome to the lounge");
// }

// Unit Structs
struct Dog;
struct Cat;

#[derive(Default)]
struct Cow {
    name: String,
}

impl Cow {
    fn has_horns(&self) -> bool {
        true
    }
}

impl Dog {
    fn does_bark(&self) -> bool{
        true
    }

    fn likes_bones(&self) -> bool {
        true
    }
}

// impl Trait for Type. ex: impl Human for Student, impl Human for Employee
impl Animal for Dog {
    fn sound(&self) {
        println!("Woof Woof");
    }

    fn num_legs(&self) {
        println!("4");
    }

    fn num_eyes(&self) {
        println!("2");
    }

    // fn func() {
    //     print!("weohfwegr");
    // }
}

impl Animal for Cow {
    fn sound(&self) {
        println!("Moo Moo");
    }

    fn num_legs(&self) {
        println!("4");
    }

    fn num_eyes(&self) {
        println!("2");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("Meow Meow");
    }

    fn num_legs(&self) {
        println!("4");
    }

    fn num_eyes(&self) {
        println!("2");
    }

}

fn animal_example() {
    let dog = Dog;
    let cat = Cat;
    let cow = Cow {
        name: String::from("Gauri"),
    };

    dog.does_bark();
    dog.likes_bones();

    dog.sound();
    dog.num_legs();
    dog.num_eyes();

    // Dog::type_of_animal();
}

trait Animal {
    fn sound(&self);
    fn num_legs(&self);
    fn num_eyes(&self);
    // fn type_of_animal();
}

trait Feline {
    // Already Defined.
    // Default Definition.
    fn meow() {
        println!("Meow Meow");
    }
}

impl Feline for Cat {}
// impl Feline for Cat {
//     fn meow() {
//         println!("A Different Meow");
//     }
// }

// For something(of some defined type) to be printed on the console, they must implement "Display" trait
// For something(of some defined type) to be debug printed on the console, they must implement "Debug" trait

// i32 is printed
fn print_i32(value: i32) {
    println!("{:?}", value);
}

// u8 is Printed
fn print_u8(value: u8) {
    println!("{:?}", value);
}

// impl Display for Dog {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Dog")
//     }
// }

impl Display for Cow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// impl Debug for Dog {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Dog")
//     }
// }

impl Debug for Cow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// impl Debug for Dog
// fn print_dog(value: Dog) {
//     println!("{:?}", value);
// }

// impl Display for Dog
// fn print_dog(value: Dog) {
//     println!("{}", value);
// }

// fn hello_dog(value: Dog) {
//     println!("Hello from Dog!");
// }

// fn print_cat(value: Cat) {
//     println!("{}", value);
// }

// fn print_cow(value: Cow) {
//     println!("{}", value);
// }

// String is Printed
fn hello_cow(value: Cow) {
    println!("Hello from {}", value.name);
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

fn test_printing() {
    let cow = Cow {
        name: String::from("Gauri"),
    };
    print_i32(34);
    print_u8(100);
    // print_dog(Dog);
    // print_cat(Cat);
    // print_cow(cow);
    hello_cow(Cow {
        name: String::from("Gauri"),
    });
    // generic_function(true);
    // generic_function(60.45);
}

// Universal Set = All types
// Allowed Set = Types that implement the Debug trait


// 2000 *2
// 10 *4
// 4040 Rs

// Arcade - Only 10 Rupees notes accepted
// 40

// T: Debug - Trait Bound
fn generic_function<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value); // It is asking for a trait bound
}

// fn generic_function<T: Display>(value: T) {
//     println!("{}", value); // It is asking for a trait bound
// }

fn advance_generic_function<T: Display + Debug>(value: T) {
    println!("{:?}", value);
    println!("{}", value);
}

fn test_trait_bounds() {
    advance_generic_function(34);
    advance_generic_function(true);
    advance_generic_function(60.45);
    advance_generic_function(Cow {
        name: String::from("Gauri"),
    });
}

// Dynamic Dispatch - The compiler does not know at compile time which function to call. It is decided in the Runtime.
// Static Dispatch - The compiler knows at compile time which function to call.

fn dyn_dispatch_example() {
    let cow = Cow {
        name: String::from("Gauri"),
    };
    
    cow.sound();

    let array_of_cows: [Cow; 5] = [Cow::default(), Cow::default(), Cow::default(), Cow::default(), Cow::default()];
    for cow in array_of_cows.iter() {
        cow.sound();
    }

    // Monomorphization

    // Smart Pointers - Box / &
    // Reference[Pointer]
    let array_of_animals: [&dyn Animal; 5] = [&Dog, &Dog, &Cat, &Cat, &Cow::default()];
    for animal in array_of_animals.iter() {
        animal.sound();
    }
    // Any type that implements the Animal trait can be used to call the sound function.
}