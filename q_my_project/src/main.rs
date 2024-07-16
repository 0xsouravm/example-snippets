mod my_module;

fn main() {
    my_module::greet();
}

// module are used to organize code inside one crate

// two types of modules
// 1. File modules
// 2. Inline module


// Inline module

// mod my_module{
//     pub fn greet() {
//         println!("Hello from my_module");
//     }
// }