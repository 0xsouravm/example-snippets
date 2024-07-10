// Traits: allow types to implement certain methods that extend their functionality
// To Restrict

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

struct Student {
    name: String
}

impl Student {
    fn new(name: String) -> Student {
        Student {
            name
        }
    }

    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn random() {
        println!("Random");
    }
}

fn abc() {
    let student = Student::new(String::from("John"));
    student.print_name();
}